use std::cmp::Ordering;
use std::fs;

#[macro_use]
extern crate scan_fmt;

#[derive(Clone, Copy, Debug)]
struct Planet {
    i: usize,
    x: i32,
    y: i32,
    z: i32,
    dx: i32,
    dy: i32,
    dz: i32,
}

impl Planet {
    fn energy(&self) -> u32 {
        println!("{:?}", self);
        self.potential() * self.kinetic()
    }
    fn potential(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }
    fn kinetic(&self) -> u32 {
        (self.dx.abs() + self.dy.abs() + self.dz.abs()) as u32
    }
    fn mv(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }
}

fn do_delta(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn do_gravity(p: &mut Vec<Planet>) {
    let bv = p.to_vec();
    let ai = p.iter_mut();

    for mut a in ai {
        for b in &bv {
            if a.i == b.i {
                continue;
            }
            a.dx += do_delta(a.x, b.x);
            a.dy += do_delta(a.y, b.y);
            a.dz += do_delta(a.z, b.z);
        }
    }
}

fn do_velocity(p: &mut Vec<Planet>) {
    p.iter_mut().for_each(|p| p.mv());
}

fn calc_energy(p: &Vec<Planet>) -> u32 {
    let mut sum = 0;
    for pi in p {
        sum += pi.energy();
    }
    sum
}

fn loop_planets(mut planets: &mut Vec<Planet>, i: usize) -> u32 {
    let mut energy = 0;
    for n in 0..i {
        println!("{}: {:?}", n, planets);
        do_gravity(&mut planets);
        do_velocity(&mut planets);
        energy = calc_energy(&planets);
    }
    energy
}

fn main() {
    let input = fs::read_to_string("day12.txt").unwrap();

    let mut planets: Vec<Planet> = input
        .lines()
        .map(|x| scan_fmt!(x, "<x={d}, y={d}, z={d}>", i32, i32, i32).unwrap())
        .enumerate()
        .map(|(i, (x, y, z))| Planet {
            i: i,
            x: x,
            y: y,
            z: z,
            dx: 0,
            dy: 0,
            dz: 0,
        })
        .collect();

    let energy = loop_planets(&mut planets, 1000);
    println!("{}", energy);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_1() {
        let io = Planet{i:0,x:-1,y:0,z:2,dx:0,dy:0,dz:0};
        let europa = Planet{i:1,x:2,y:-10,z:-7,dx:0,dy:0,dz:0};
        let ganymede = Planet{i:2,x:4,y:-8,z:8,dx:0,dy:0,dz:0};
        let callisto = Planet{i:3,x:3,y:5,z:-1,dx:0,dy:0,dz:0};
        let mut planets = vec![io, europa, ganymede, callisto];
        let energy = loop_planets(&mut planets, 10);
        assert_eq!(179, energy);
    }
}
