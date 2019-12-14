use num::integer::gcd;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32;
use std::fs;

fn to_angle(x: i32, y: i32) -> f32 {
    f32::atan2(y as f32, x as f32) * 180.0 / f32::consts::PI
}

fn one_two(s: &str) -> (usize, i32) {
    let mut map: Vec<(i32, i32)> = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            if c == '#' {
                map.push((x as i32, y as i32));
                width = width.max(x + 1);
                height = height.max(y + 1);
            }
        }
    }

    let mut pos = Vec::new();

    for a in map.iter() {
        let mut gcd_set = HashSet::new();
        for b in map.iter() {
            if a == b {
                continue;
            }
            let (ax, ay) = a;
            let (bx, by) = b;
            let dx = ax - bx;
            let dy = ay - by;
            let g = gcd(dx, dy);
            let gdx = dx / g;
            let gdy = dy / g;
            gcd_set.insert((gdx, gdy));
        }
        pos.push((gcd_set.len(), a));
    }

    let (best, best_pos) = pos.iter().max().unwrap();

    if map.len() < 200 {
        println!("{}", map.len());
        return (*best, 0);
    }

    println!("Best pos = {:?}", best_pos);

    let (bx, by) = best_pos;
    let mut asteroids = HashMap::new();
    for a in map.iter().cloned() {
        let (ax, ay) = a;
        if ax == *bx && ay == *by {
            continue;
        }
        let dx = bx - ax;
        let dy = by - ay;
        let distance = ((dx * dx + dy * dy) as f32).sqrt();
        let g = gcd(dx, dy);
        let gdx = dx / g;
        let gdy = dy / g;
        let angle = (to_angle(gdx, gdy) - 90.0).rem_euclid(360.0);
        println!("({}, {}), ({}, {}), {}, {},{} = {}", bx, by, ax, ay, g, gdx, gdy, angle);
        let angle_i = (angle * 100.0).round() as i32;
        asteroids
            .entry(angle_i)
            .or_insert(Vec::new())
            .push((distance, a));
    }

    asteroids
        .values_mut()
        .for_each(|x| x.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Equal)));

    let mut asteroids:Vec<(i32,Vec<(f32, (i32, i32))>)> = asteroids.iter().map(|(k,v)|(k.clone(),v.clone())).collect();
    asteroids.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

    let mut n = 0;
    let mut best_pos = 0;

    loop {
        for (a, v) in asteroids.iter_mut() {
            if v.len() > 0 {
                n += 1;
                let (d, (x, y)) = v.pop().unwrap();
                println!("{}, x={}, y={}, a={}, d={}", n, x, y, a, d);
                if n == 200 {
                    best_pos = x * 100 + y;
                }
            }
        }

        if best_pos != 0 {
            break;
        }
    }

    (*best, best_pos)
}

fn main() {
    let input = fs::read_to_string("day10.txt").unwrap();
    let (one, two) = one_two(&input);
    println!("{}", one);
    println!("{}", two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_1() {
        let input = ".#..#
        .....
        #####
        ....#
        ...##";
        let (ret, _) = one_two(&input);

        assert_eq!(8, ret);
    }
    #[test]
    fn test_two_1() {
        let input = ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";

        let (one, two) = one_two(&input);

        assert_eq!(210, one);
        assert_eq!(802, two);
    }
}
