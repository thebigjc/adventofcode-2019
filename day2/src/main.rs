use std::fs;

fn parse(s: &str) -> usize {
    s.trim().parse().unwrap()
}

fn main() {
    let mut tokens:Vec<usize> = fs::read_to_string("day2.txt").unwrap().split(",").map(parse).collect();

    let mut i = 0;

    tokens[1] = 12;
    tokens[2] = 2;

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
                println!("99");
                break;
            }
            _ => {
                println!("Oops: {} {}", i, tokens[i]);
            }
        }
    }

    println!("{}", tokens[0]);
}
