use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn all_fuel(mass: i32) -> i32 {
    let f = fuel(mass);
    if f > 0 {
        f + all_fuel(f)
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let f = File::open("day1.txt")?;
    let f = BufReader::new(f);
    let mut sum = 0;
    let mut all_sum = 0;

    for line in f.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        sum += fuel(mass);
        all_sum += all_fuel(mass)
    }

    println!("{}", sum);
    println!("{}", all_sum);

    Ok(())
}
