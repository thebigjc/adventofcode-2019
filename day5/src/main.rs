use std::fs;

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
        _ => panic!("{}", m)
    }
}

fn intcode(program: &Vec<i32>, input: i32) -> i32 {
    let mut i = 0;
    let mut output = 0;
    let mut tokens = program.to_vec();

    loop {
        let t = tokens[i];
        let opcode = t % 100;
        let a_mode = mode((t / 100) % 10);
        let b_mode = mode((t / 1000) % 10);

        println!("a_mode = {}, t = {}, opcode = {}", (t / 100) % 10, t, opcode);

        match opcode {
            1 => {
                let a = a_mode(&tokens, i+1);
                let b = b_mode(&tokens, i+2);
                let c = tokens[i+3] as usize;
                tokens[c] = a+b;
                i += 4;
            }
            2 => {
                let a = a_mode(&tokens, i+1);
                let b = b_mode(&tokens, i+2);
                let c = tokens[i+3] as usize;
                tokens[c] = a*b;
                i += 4;
            }
            3 => {
                let a = tokens[i+1] as usize;
                tokens[a] = input;
                i += 2;
            }
            4 => {
                output = a_mode(&tokens, i+1);
                i += 2;
                if tokens[i] != 99 && output != 0 {
                    panic!("Failed test: i = {}, output = {}", tokens[i], output);
                }
            }
            5 => {
                output = a_mode(&tokens, i+1);
                let a = a_mode(&tokens, i+1);
                let b = b_mode(&tokens, i+2);
                if a != 0 {
                    i = b as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                output = a_mode(&tokens, i+1);
                let a = a_mode(&tokens, i+1);
                let b = b_mode(&tokens, i+2);
                if a == 0 {
                    i = b as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                output = a_mode(&tokens, i+1);
                let a = a_mode(&tokens, i+1);
                let b = b_mode(&tokens, i+2);
                let c = tokens[i+3] as usize;
                if a < b {
                    tokens[c] = 1;
                } else {
                    tokens[c] = 0;
                }
                i+=4;
            }
            8 => {
                output = a_mode(&tokens, i+1);
                let a = a_mode(&tokens, i+1);
                let b = b_mode(&tokens, i+2);
                let c = tokens[i+3] as usize;
                if a == b {
                    tokens[c] = 1;
                } else {
                    tokens[c] = 0;
                }
                i+=4;
            }
            99 => {
                break;
            }
            _ => {
                println!("Oops: {} {}", i, tokens[i]);
            }
        }
    }

    output
}

fn main() {
    let tokens:Vec<i32> = fs::read_to_string("day5.txt").unwrap().split(",").map(parse).collect();
    let ret = intcode(&tokens, 1);
    println!("{}", ret);
    let ret = intcode(&tokens, 5);
    println!("{}", ret);
}
