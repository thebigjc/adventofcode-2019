use std::cmp::Ordering;
use std::fs;

#[macro_use]
extern crate scan_fmt;

#[derive(Clone, Copy, Debug)]
struct Planet {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn gcd(a: usize, b: usize) -> usize {
    return if b == 0 { a } else { gcd(b, a % b) };
}

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

impl Planet {
    fn energy(&self) -> usize {
        self.potential() * self.kinetic()
    }
    fn potential(&self) -> usize {
        self.pos.iter().map(|x| x.abs()).sum::<i64>() as usize
    }
    fn kinetic(&self) -> usize {
        self.vel.iter().map(|x| x.abs()).sum::<i64>() as usize
    }
    fn mv(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }
}

fn do_delta(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn do_gravity(p: &mut Vec<Planet>) {
    for i in 0..p.len() {
        for j in i+1..p.len() {
            for n in 0..3 {
                p[i].vel[n] += do_delta(p[i].pos[n], p[j].pos[n]);
                p[j].vel[n] += do_delta(p[j].pos[n], p[i].pos[n]);
            }
        }
    }
}

fn do_velocity(p: &mut Vec<Planet>) {
    p.iter_mut().for_each(|p| p.mv());
}

fn calc_energy(p: &Vec<Planet>) -> usize {
    let mut sum = 0;
    for pi in p {
        sum += pi.energy();
    }
    sum
}

fn loop_planets(mut planets: &mut Vec<Planet>, i: usize) -> usize {
    let mut energy = 0;
    for _ in 0..i {
        do_gravity(&mut planets);
        do_velocity(&mut planets);
        energy = calc_energy(&planets);
    }
    energy
}

fn loop_planets_dupe(mut planets: &mut Vec<Planet>) -> usize {
    let mut cnt = 0;
    let mut period: [usize; 3] = [0, 0, 0];

    loop {
        cnt += 1;
        do_gravity(&mut planets);
        for i in 0..3 {
            if period[i] == 0 && planets.iter().all(|p| p.vel[i] == 0) {
                println!("x[{}] = 0, cnt = {}", i, cnt);
                period[i] = cnt;
            }
        }
        if period.iter().all(|x| *x > 0) {
            break;
        }

        do_velocity(&mut planets);
    }
    println!("{:?}", period);
    lcm(period[0], lcm(period[1], period[2])) * 2
}

fn main() {
    let input = fs::read_to_string("day12.txt").unwrap();

    let mut planets: Vec<Planet> = input
        .lines()
        .map(|x| scan_fmt!(x, "<x={d}, y={d}, z={d}>", i64, i64, i64).unwrap())
        .map(|(x, y, z)| Planet {
            pos: [x, y, z],
            vel: [0, 0, 0],
        })
        .collect();

    let energy = loop_planets(&mut planets, 1000);
    println!("{}", energy);
    let cnt = loop_planets_dupe(&mut planets);
    println!("{}", cnt);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_planets() -> Vec<Planet> {
        let io = Planet {
            pos: [-1, 0, 2],
            vel: [0, 0, 0],
        };
        let europa = Planet {
            pos: [2, -10, -7],
            vel: [0, 0, 0],
        };
        let ganymede = Planet {
            pos: [4, -8, 8],
            vel: [0, 0, 0],
        };
        let callisto = Planet {
            pos: [3, 5, -1],
            vel: [0, 0, 0],
        };
        let planets = vec![io, europa, ganymede, callisto];
        planets
    }

    fn mk_planets_2() -> Vec<Planet> {
        let io = Planet {
            pos: [-8, -10, 0],
            vel: [0, 0, 0],
        };
        let europa = Planet {
            pos: [5, 5, 10],
            vel: [0, 0, 0],
        };
        let ganymede = Planet {
            pos: [2, -7, 3],
            vel: [0, 0, 0],
        };
        let callisto = Planet {
            pos: [9, -8, -3],
            vel: [0, 0, 0],
        };
        let planets = vec![io, europa, ganymede, callisto];
        planets
    }

    #[test]
    fn test_one_1() {
        let mut planets = mk_planets();
        let energy = loop_planets(&mut planets, 10);
        assert_eq!(179, energy);
    }
    #[test]
    fn test_two_1() {
        let mut planets = mk_planets();
        let cnt = loop_planets_dupe(&mut planets);
        assert_eq!(2772, cnt);
    }
    #[test]
    fn test_two_2() {
        let mut planets = mk_planets_2();
        let cnt = loop_planets_dupe(&mut planets);
        assert_eq!(4686774924, cnt);
    }
}
