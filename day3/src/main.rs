use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

#[derive(Debug)]
enum Dir {
    U,D,L,R
}

struct DirLen {
    d: Dir,
    l: u16,
}

fn parse(s: &str) -> DirLen {
    let dc = s.chars().next().unwrap();
    let d = match dc {
        'U' => Dir::U,
        'D' => Dir::D,
        'L' => Dir::L,
        'R' => Dir::R,
        _ => panic!("Ummm...what?")
    };
    let l = s[1..s.len()].parse().unwrap();
    DirLen{d:d,l:l}
}

fn parse_line(line: &str) -> Vec<DirLen> {
    line.trim().split(",").map(parse).collect()
}

fn build_set(v: Vec<DirLen>) -> HashSet<(i32,i32)> {
    let mut s: HashSet<(i32,i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    for d in v {
        let (xd,yd) = match d.d {
            Dir::U => (0,1),
            Dir::D => (0,-1),
            Dir::L => (-1,0),
            Dir::R => (1,0),
        };
        for _i in 0..d.l {
            x += xd;
            y += yd;
            s.insert((x,y));
        }
    }

    s
}

fn manhattan(p: &(i32,i32)) -> i32 {
    let (a, b) = p;
    a.abs() + b.abs()
}

fn one(l1: &str, l2: &str) -> i32 {
    let l1:Vec<DirLen> = parse_line(l1);
    let l2:Vec<DirLen> = parse_line(l2);

    let s1 = build_set(l1);
    let s2 = build_set(l2);

    println!("{}", s1.len());
    println!("{}", s2.len());

    let i = s1.intersection(&s2);
    let iv = i.map(manhattan).collect::<Vec<i32>>();

    *iv.iter().min().unwrap()
}

fn main() {
    let f = File::open("day3.txt").unwrap();
    let mut f = BufReader::new(f);

    let mut l1 = String::new();
    f.read_line(&mut l1).expect("can't fail to read line");

    let mut l2 = String::new();
    f.read_line(&mut l2).expect("can't fail to read line");

    println!("{}", one(&l1, &l2));

    //Ok(())
}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn simple() {
        let s1 = "R8,U5,L5,D3";
        let s2 = "U7,R6,D4,L4";
        let r = one(&s1, &s2);
        assert_eq!(6, r);
    }
    #[test]
    fn test1() {
        let s1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let s2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        let r = one(&s1, &s2);
        assert_eq!(159, r);
    }
    #[test]
    fn test2() {
        let s1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let s2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let r = one(&s1, &s2);
        assert_eq!(135, r);
    }
}
