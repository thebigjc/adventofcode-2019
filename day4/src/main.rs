use std::iter::Iterator;
use std::iter::FromIterator;
use std::collections::HashMap;

fn main() {
   let ret = one(271973, 785961); 
   println!("{}", ret);
   let ret = two(271973, 785961); 
   println!("{}", ret);
}

fn has_pair(s: &String) -> bool {
    let mut count = HashMap::new();

    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    for v in count.values() {
        if *v == 2 {
            return true;
        }
    }

    false
}

fn has_dup(s: &String) -> bool {
    let mut v:Vec<char> = s.chars().collect();

    v.dedup();

    v.len() < s.len()
}

fn valid_one(x: u32) -> bool {
    valid(x, &has_dup)
}

fn valid_two(x: u32) -> bool {
    valid(x, &has_pair)
}


fn valid(x: u32, f: &dyn Fn(&String) -> bool) -> bool {
    let s1: String = x.to_string();
    let mut chars: Vec<char> = s1.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    let s2 = String::from_iter(chars);
    let d = f(&s2);
    let e = s1 == s2;
    let l = s1.len() == 6;
    //println!("d = {}, e = {}, l = {}, s1 = {}, s2 = {}", d, e, l, s1, s2);

    l && e && d
}

fn one(a: u32, b: u32) -> u32 {
    let mut n = 0;

    for i in a..=b {
        if valid_one(i) {
            n+=1;
        }
    }
    n
}

fn two(a: u32, b: u32) -> u32 {
    let mut n = 0;

    for i in a..=b {
        if valid_two(i) {
            n+=1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn test1() {
        assert!(valid_one(111111));
        assert!(!valid_one(223450));
        assert!(!valid_one(123789));
        assert!(valid_one(111123));
        assert!(valid_one(122345));
        assert!(!valid_one(135679));
    }
    #[test]
    fn test2() {
        assert!(valid_two(112233));
        assert!(!valid_two(123444));
        assert!(valid_two(111122));
    }
}
