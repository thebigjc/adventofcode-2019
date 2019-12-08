use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

fn parse(c: char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Bad number"),
    }
}

fn one(nums: Vec<i32>, width: usize, height: usize) -> usize {
    println!("{} nums, {} layersize", nums.len(), width*height);
    let chunks:Vec<&[i32]> = nums.chunks(width*height).filter(|x|x.len() == width*height).collect();
    println!("{}", chunks.len());
    let zeros:Vec<usize> = chunks.iter().map(|x|x.iter().filter(|y|**y==0).count()).collect();
    let (min_zeros, min_chunk) = zeros.iter().zip(chunks).min_by_key(|(x,_)|*x).unwrap();
    let mut min_chunk = min_chunk.to_vec();
    min_chunk.sort();

    let mut m = HashMap::new();

    for (k, g) in &min_chunk.into_iter().group_by(|x|*x) {
        m.insert(k, g.count());
    }

    let one = *(m.get(&1).unwrap());
    let two = *(m.get(&2).unwrap());
    let ret = one * two;

    println!("{} * {} = {}", one, two, ret);

    ret

}

fn main() {
    let nums: Vec<i32> = fs::read_to_string("day8.txt").unwrap().trim().chars().map(parse).collect();
    let l = one(nums, 25, 6);
    println!("{:?}", l);
}
