use itertools::Itertools;
use std::fs;
use std::i32;
use std::collections::VecDeque;

fn parse(s: &str) -> i32 {
    s.trim().parse().unwrap()
}

fn position(t: &Vec<i32>, i: usize) -> i32 {
    t[t[i] as usize] as i32
}

fn immediate(t: &Vec<i32>, i: usize) -> i32 {
    t[i] as i32
}

fn mode(m: i32) -> fn(&Vec<i32>, usize) -> i32 {
    match m {
        0 => position,
        1 => immediate,
        _ => panic!("{}", m),
    }
}

struct Program {
    orig: Vec<i32>,
    tokens: Vec<i32>,
    inputs: VecDeque<i32>,
    output: i32,
    i: usize,
    halted: bool,
}

impl Program {
    fn new(t: &Vec<i32>) -> Program {
        Program {
            orig: t.to_vec(),
            tokens: t.to_vec(),
            inputs: VecDeque::new(),
            output: i32::MIN,
            i: 0,
            halted: false,
        }
    }

    fn reset(&mut self) {
        self.tokens = self.orig.to_vec();
        self.i = 0;
        self.inputs = VecDeque::new();
        self.output = i32::MIN;
        self.halted = false;
    }

    fn op(
        &mut self,
        a_mode: fn(&Vec<i32>, usize) -> i32,
        b_mode: fn(&Vec<i32>, usize) -> i32,
        o: fn(a: i32, b: i32) -> i32,
    ) {
        let a = a_mode(&self.tokens, self.i + 1);
        let b = b_mode(&self.tokens, self.i + 2);
        let c = self.tokens[self.i + 3] as usize;
        self.tokens[c] = o(a, b);
        self.i += 4;
    }

    fn branch(
        &mut self,
        a_mode: fn(&Vec<i32>, usize) -> i32,
        b_mode: fn(&Vec<i32>, usize) -> i32,
        o: fn(a: i32) -> bool,
    ) {
        let a = a_mode(&self.tokens, self.i + 1);
        let b = b_mode(&self.tokens, self.i + 2);
        if o(a) {
            self.i = b as usize;
        } else {
            self.i += 3;
        }
    }
    fn test(
        &mut self,
        a_mode: fn(&Vec<i32>, usize) -> i32,
        b_mode: fn(&Vec<i32>, usize) -> i32,
        o: fn(a: i32, b: i32) -> bool,
    ) {
        let a = a_mode(&self.tokens, self.i + 1);
        let b = b_mode(&self.tokens, self.i + 2);
        let c = self.tokens[self.i + 3] as usize;
        if o(a, b) {
            self.tokens[c] = 1;
        } else {
            self.tokens[c] = 0;
        }
        self.i += 4;
    }

    fn intcode(&mut self) {
        loop {
            let t = self.tokens[self.i];
            let opcode = t % 100;
            let a_mode = mode((t / 100) % 10);
            let b_mode = mode((t / 1000) % 10);

            //        println!("a_mode = {}, t = {}, opcode = {}", (t / 100) % 10, t, opcode);

            match opcode {
                1 => {
                    self.op(a_mode, b_mode, |a, b| a + b);
                }
                2 => {
                    self.op(a_mode, b_mode, |a, b| a * b);
                }
                3 => {
                    let a = self.tokens[self.i + 1] as usize;
                    self.tokens[a] = self.inputs.pop_front().unwrap();
                    self.i += 2;
                }
                4 => {
                    self.output = a_mode(&self.tokens, self.i + 1);
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

fn run(programs: &mut Vec<Program>, order: &Vec<i32>) -> i32 {
    programs.iter_mut().for_each(|x|x.reset());
    for (x,p) in order.iter().zip(programs.iter_mut()) {
        p.inputs.push_back(*x);
    }

    //println!("Order:{:?}", order);
    let mut output = 0;
    let mut i = 0;
    while !programs.iter().any(|x|x.halted) {
        i += 1;
        if i > 10000 {
            panic!("Loops");
        }
        for p in programs.iter_mut() {
            p.inputs.push_back(output);
            p.intcode();
            output = p.output;
        }
    }
    //println!("Order:{:?}={}", order, output);
    output
}

fn one(t: &Vec<i32>) -> i32 {
    let mut programs:Vec<Program> = Vec::new();

    for _ in 0..5 {
        programs.push(Program::new(t));
    }

    (0..=4).permutations(5).map(|x| run(&mut programs, &x)).max().unwrap()
}

fn two(t: &Vec<i32>) -> i32 {
    let mut programs:Vec<Program> = Vec::new();

    for _ in 0..5 {
        programs.push(Program::new(t));
    }

    (5..=9).permutations(5).map(|x| run(&mut programs, &x)).max().unwrap()
}

fn main() {
    let tokens: Vec<i32> = fs::read_to_string("day7.txt")
        .unwrap()
        .split(",")
        .map(parse)
        .collect();
    let ret = one(&tokens);
    println!("one = {}", ret);
    let ret = two(&tokens);
    println!("two = {}", ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_1() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let ret = one(&input);

        assert_eq!(43210, ret);
    }
    #[test]
    fn test_one_2() {
        let input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let ret = one(&input);

        assert_eq!(54321, ret);
    }
    #[test]
    fn test_one_3() {
        let input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let ret = one(&input);

        assert_eq!(65210, ret);
    }
    #[test]
    fn test_two_1() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let ret = two(&input);

        assert_eq!(139629729, ret);
    }
    #[test]
    fn test_two_2() {
        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let ret = two(&input);

        assert_eq!(18216, ret);
    }
}
