use std::fs;

fn parse(s: &str) -> usize {
    s.trim().parse().unwrap()
}

fn intcode(input: &Vec<usize>, a: usize, b: usize) -> usize {
    let mut i = 0;
    let mut tokens = input.to_vec();

    tokens[1] = a;
    tokens[2] = b;

    loop {
        match tokens[i] {
            1 => {
                let a = tokens[tokens[i+1]];
                let b = tokens[tokens[i+2]];
                let c = tokens[i+3];
                tokens[c] = a+b;
                i += 4;
            }
            2 => {
                let a = tokens[tokens[i+1]];
                let b = tokens[tokens[i+2]];
                let c = tokens[i+3];
                tokens[c] = a*b;
                i += 4;
            }
            99 => {
                break;
            }
            _ => {
                println!("Oops: {} {}", i, tokens[i]);
            }
        }
    }

    tokens[0]
}

fn main() {
    let tokens:Vec<usize> = fs::read_to_string("day2.txt").unwrap().split(",").map(parse).collect();
    let ret = intcode(&tokens, 12, 2);

    println!("{}", ret);

    for a in 0..=99 {
        for b in 0..=99 {
            let ret = intcode(&tokens, a, b);
            if ret == 19690720 {
                println!("100 * {} + {} = {}", a, b, 100*a+b);
                break;
            }
        }
    }
}
