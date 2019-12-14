use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::i128;
use std::iter::FromIterator;
use std::cmp::Ordering;

fn parse(s: &str) -> i128 {
    s.trim().parse().unwrap()
}

struct Program {
    tokens: HashMap<i128, i128>,
    inputs: VecDeque<i128>,
    output: i128,
    i: i128,
    rb: i128,
    halted: bool,
}

impl Program {
    fn new(t: &Vec<i128>) -> Program {
        Program {
            tokens: HashMap::from_iter(t.iter().enumerate().map(|(a, b)| (a as i128, *b))),
            inputs: VecDeque::new(),
            output: i128::MIN,
            i: 0,
            rb: 0,
            halted: false,
        }
    }

    fn position(&self, o: i128) -> i128 {
        let i = *self.tokens.get(&(self.i + o)).unwrap_or(&0);

        *self.tokens.get(&i).unwrap_or(&0)
    }

    fn immediate(&self, o: i128) -> i128 {
        *self.tokens.get(&(self.i + o)).unwrap_or(&0)
    }

    fn relative(&self, o: i128) -> i128 {
        let pos = self.tokens.get(&(self.i + o)).unwrap_or(&0);
        let i = (self.rb + pos) as i128;

        *self.tokens.get(&i).unwrap_or(&0)
    }

    fn mode(&self, m: i128) -> fn(&Program, i128) -> i128 {
        match m {
            0 => Program::position,
            1 => Program::immediate,
            2 => Program::relative,
            _ => panic!("{}", m),
        }
    }

    fn store(&mut self, i: i128, x: i128) {
        //println!("Store: {} = {}", i, x);
        self.tokens.insert(i, x);
    }

    fn op(
        &mut self,
        a_mode: fn(&Program, i128) -> i128,
        b_mode: fn(&Program, i128) -> i128,
        rb: i128,
        o: fn(a: i128, b: i128) -> i128,
    ) {
        let a = a_mode(self, 1);
        let b = b_mode(self, 2);
        let c = self.immediate(3) + rb;
        self.store(c, o(a, b));
        self.i += 4;
    }

    fn branch(
        &mut self,
        a_mode: fn(&Program, i128) -> i128,
        b_mode: fn(&Program, i128) -> i128,
        o: fn(a: i128) -> bool,
    ) {
        let a = a_mode(self, 1);
        let b = b_mode(self, 2);
        if o(a) {
            self.i = b as i128;
        } else {
            self.i += 3;
        }
    }
    fn test(
        &mut self,
        a_mode: fn(&Program, i128) -> i128,
        b_mode: fn(&Program, i128) -> i128,
        rb: i128,
        o: fn(a: i128, b: i128) -> bool,
    ) {
        let a = a_mode(self, 1);
        let b = b_mode(self, 2);
        let c = self.immediate(3) + rb;
        if o(a, b) {
            self.store(c, 1);
        } else {
            self.store(c, 0);
        }
        self.i += 4;
    }

    fn intcode(&mut self) {
        loop {
            let t = *self.tokens.get(&self.i).unwrap_or(&0);
            let opcode = t % 100;
            let a_mode = self.mode((t / 100) % 10);
            let b_mode = self.mode((t / 1000) % 10);
            let am = (t / 100) % 10;
            let cm = (t / 10000) % 10;
            let mut rb = 0;
            if cm == 2 {
                rb = self.rb;
            }

            match opcode {
                1 => {
                    self.op(a_mode, b_mode, rb, |a, b| a + b);
                }
                2 => {
                    self.op(a_mode, b_mode, rb, |a, b| a * b);
                }
                3 => {
                    let mut a = self.immediate(1);
                    if am == 2 {
                        a += self.rb;
                    }
                    let i = self.inputs.pop_front().unwrap();
                    self.store(a, i);
                    self.i += 2;
                }
                4 => {
                    self.output = a_mode(self, 1);
                    self.i += 2;
                    return;
                }
                5 => {
                    self.branch(a_mode, b_mode, |a| a != 0);
                }
                6 => {
                    self.branch(a_mode, b_mode, |a| a == 0);
                }
                7 => {
                    self.test(a_mode, b_mode, rb, |a, b| a < b);
                }
                8 => {
                    self.test(a_mode, b_mode, rb, |a, b| a == b);
                }
                9 => {
                    let x = a_mode(self, 1);
                    self.rb += x;
                    self.i += 2;
                }
                99 => {
                    println!("Halt!");
                    self.halted = true;
                    return;
                }
                _ => {
                    println!("Oops: {} {}", self.i, self.tokens[&self.i]);
                }
            }
        }
    }
}

fn one(t: &Vec<i128>) -> usize {
    paint(t,false)
}

fn two(t: &Vec<i128>) -> usize {
    paint(t, true)
}

fn paint(t: &Vec<i128>, play: bool) -> usize {
    let mut p = Program::new(t);
    if play {
        p.tokens.insert(0,2);
    }
    let mut tiles = HashMap::new();
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        let dir = match ball_x.cmp(&paddle_x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1
        };
        if p.inputs.len() == 0 {
            p.inputs.push_front(dir); 
        } else {
            p.inputs[0] = dir; 
        }

        p.intcode();
        if p.halted {
            break;
        }

        let x = p.output;
        p.intcode();
        let y = p.output;
        p.intcode();
        let c = p.output;
        if x == -1 && y == 0 {
            score = c;
        } else {
            if c == 4 {
                ball_x = x;
            }
            if c == 3 {
                paddle_x = x;
            }
            tiles.insert((x,y), c);
        }
    }
    if play {
        score as usize
    } else {
        tiles.values().filter(|x|**x==2).count()
    }
}

fn main() {
    let tokens: Vec<i128> = fs::read_to_string("day13.txt")
        .unwrap()
        .split(",")
        .map(parse)
        .collect();
    let ret = one(&tokens);
    println!("one = {:?}", ret);
    let ret = two(&tokens);
    println!("two = {:?}", ret);
}

