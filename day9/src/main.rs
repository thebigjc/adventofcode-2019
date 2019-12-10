use std::fs;
use std::i128;
use std::collections::VecDeque;

fn parse(s: &str) -> i128 {
    s.trim().parse().unwrap()
}


struct Program {
    orig: Vec<i128>,
    tokens: Vec<i128>,
    inputs: VecDeque<i128>,
    output: i128,
    i: usize,
    rb: i128,
    halted: bool,
}

impl Program {
    fn new(t: &Vec<i128>) -> Program {
        Program {
            orig: t.to_vec(),
            tokens: t.to_vec(),
            inputs: VecDeque::new(),
            output: i128::MIN,
            i: 0,
            rb : 0,
            halted: false,
        }
    }

    fn position(&mut self, o: usize) -> i128 {
        if self.i+o >= self.tokens.len() {
            self.tokens.resize(self.i+o+1, 0);
        }
        let i = self.tokens[self.i+o] as usize;
        if i >= self.tokens.len() {
            self.tokens.resize(i+1, 0);
        }
         
        self.tokens[self.tokens[self.i+o] as usize]
    }

    fn immediate(&mut self, o: usize) -> i128 {
        if self.i+o >= self.tokens.len() {
            self.tokens.resize(self.i+o+1,0);
        }
        self.tokens[self.i+o]
    }

    fn relative(&mut self, o: usize) -> i128 {
        println!("Relative o: {}", o);
        if self.i+o >= self.tokens.len() {
            self.tokens.resize(self.i+o+1, 0);
        }
        println!("Relative self.i+o: {}", self.i+o);
        println!("Relative toknes[self.i+o]: {}", self.tokens[self.i+o]);
        let i = (self.tokens[self.i+o] + self.rb) as usize;
        if i >= self.tokens.len() {
            self.tokens.resize(i+1, 0);
        }
        println!("Relative - i: {}", i);
        println!("Relative - tokens[i]: {}", self.tokens[i]);
         
        self.tokens[i]
    }

    fn mode(&self, m: i128) -> fn(&mut Program, usize) -> i128 {
        match m {
            0 => Program::position,
            1 => Program::immediate,
            2 => Program::relative,
            _ => panic!("{}", m),
        }
    }

    fn reset(&mut self) {
        self.tokens = self.orig.to_vec();
        self.i = 0;
        self.rb = 0;
        self.inputs = VecDeque::new();
        self.output = i128::MIN;
        self.halted = false;
    }

    fn store(&mut self, i: usize, x: i128) {
        if i >= self.tokens.len() {
            self.tokens.resize(i+1, 0);
        }
        self.tokens[i] = x;
    }

    fn op(
        &mut self,
        a_mode: fn(&mut Program, usize) -> i128,
        b_mode: fn(&mut Program, usize) -> i128,
        o: fn(a: i128, b: i128) -> i128,
    ) {
        let a = a_mode(self, 1);
        let b = b_mode(self, 2);
        let c = self.immediate(3) as usize;
        self.store(c, o(a,b));
        self.i += 4;
    }

    fn branch(
        &mut self,
        a_mode: fn(&mut Program, usize) -> i128,
        b_mode: fn(&mut Program, usize) -> i128,
        o: fn(a: i128) -> bool,
    ) {
        let a = a_mode(self, 1);
        let b = b_mode(self, 2);
        if o(a) {
            self.i = b as usize;
        } else {
            self.i += 3;
        }
    }
    fn test(
        &mut self,
        a_mode: fn(&mut Program, usize) -> i128,
        b_mode: fn(&mut Program, usize) -> i128,
        o: fn(a: i128, b: i128) -> bool,
    ) {
        let a = a_mode(self, 1);
        let b = b_mode(self, 2);
        let c = self.immediate(3) as usize;
        if o(a, b) {
            self.store(c, 1);
        } else {
            self.store(c, 0);
        }
        self.i += 4;
    }

    fn intcode(&mut self) {
        loop {
            let t = self.tokens[self.i];
            let opcode = t % 100;
            let a_mode = self.mode((t / 100) % 10);
            let b_mode = self.mode((t / 1000) % 10);
            println!("rb = {}, a_mode = {}, b_mode = {}, t = {}, opcode = {}", self.rb, (t / 100) % 10, (t / 1000) % 10, t, opcode);

            match opcode {
                1 => {
                    self.op(a_mode, b_mode, |a, b| a + b);
                }
                2 => {
                    self.op(a_mode, b_mode, |a, b| a * b);
                }
                3 => {
                    let a = a_mode(self, 1) as usize;
                    let i = self.inputs.pop_front().unwrap();
                    println!("{}, {}, {}", self.i, a, i);
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
                    self.test(a_mode, b_mode, |a, b| a < b);
                }
                8 => {
                    self.test(a_mode, b_mode, |a, b| a == b);
                }
                9 => {
                    self.rb += a_mode(self, 1);
                    self.i += 2;
                }
                99 => {
                    //println!("Halt!");
                    self.halted = true;
                    return;
                }
                _ => {
                    println!("Oops: {} {}", self.i, self.tokens[self.i]);
                }
            }
        }
    }
}

fn one(t: &Vec<i128>) -> Vec<i128> {
    let mut p = Program::new(t);
    let mut output = Vec::new();
    p.inputs.push_back(1);
    loop {
        p.intcode();
        if p.halted {
            break;
        }
        output.push(p.output);
    }
    output 
}

fn main() {
    let tokens: Vec<i128> = fs::read_to_string("day9.txt")
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
}
