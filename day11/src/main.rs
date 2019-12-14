use std::fs;
use std::i128;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

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
            tokens: HashMap::from_iter(t.iter().enumerate().map(|(a,b)|(a as i128, *b))),
            inputs: VecDeque::new(),
            output: i128::MIN,
            i: 0,
            rb : 0,
            halted: false,
        }
    }

    fn position(&self, o: i128) -> i128 {
        let i = *self.tokens.get(&(self.i+o)).unwrap_or(&0); 
         
        *self.tokens.get(&i).unwrap_or(&0)
    }

    fn immediate(&self, o: i128) -> i128 {
        *self.tokens.get(&(self.i+o)).unwrap_or(&0)
    }

    fn relative(&self, o: i128) -> i128 {
        let pos = self.tokens.get(&(self.i+o)).unwrap_or(&0);
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
        self.tokens.insert(i,x);
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
        self.store(c, o(a,b));
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
    let mut p = Program::new(t);
    let mut dx = 0;
    let mut dy = 1;
    let mut x = 0;
    let mut y = 0;
    let mut colors = HashMap::new();
    let mut painted = HashSet::new();

    loop {
        let c = *colors.get(&(x,y)).unwrap_or(&0);
        println!("At {}, {}, painted = {}", x, y, c);
        p.inputs.push_back(c);
        p.intcode();
        let color = p.output;
        println!("Painting {}", color);
        colors.insert((x,y), color);
        painted.insert((x,y));
        p.intcode();
        let dir = p.output;
        println!("Turning {}", dir);
        let (dx1, dy1) = match dir {
            0 => {
                match (dx, dy) { 
                    (0,1) => (-1, 0),
                    (-1,0) => (0, -1),
                    (0,-1) => (1, 0),
                    (1,0) => (0, 1),
                    _ => panic!("Unknown pattern"),
                }
            }
            1 => {
                match (dx, dy) { 
                    (0,1) => (1, 0),
                    (-1,0) => (0, 1),
                    (0,-1) => (1, 0),
                    (1,0) => (0, -1),
                    _ => panic!("Unknown pattern"),
                }
            },
            _ => panic!("Unknown dir")
        };
        dx = dx1;
        dy = dy1;
        println!("Facing {},{}", dx, dy);

        x += dx;
        y += dy;

        if p.halted {
            break;
        }
    }
    painted.len()
}

fn main() {
    let tokens: Vec<i128> = fs::read_to_string("day11.txt")
        .unwrap()
        .split(",")
        .map(parse)
        .collect();
    let ret = one(&tokens);
    println!("one = {:?}", ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_1() {
        let input = vec![
            109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99
        ];
        let mut p = Program::new(&input);
        let mut output = Vec::new();

        loop {
            p.intcode();
            if p.halted {
                break;
            }
            output.push(p.output);
        }

        assert_eq!(input, output);
    }
    #[test]
    fn test_one_2() {
        let input = vec![
            1102,34915192,34915192,7,4,7,99,0
        ];
        let mut p = Program::new(&input);
        p.intcode();
        let ret = p.output;

        assert_eq!(16, format!("{}", ret).len());
    }
    #[test]
    fn test_one_3() {
        let input = vec![
            104,1125899906842624,99
        ];
        let mut p = Program::new(&input);
        p.intcode();
        let ret = p.output;

        assert_eq!(1125899906842624, ret);
    }
    #[test]
    fn test_one_4() {
        let input = vec![
            109, 1, 203, 2, 204, 2, 99
        ];
        let mut p = Program::new(&input);
        p.inputs.push_back(1024);
        p.intcode();
        let ret = p.output;

        assert_eq!(1024, ret);
    }
}
