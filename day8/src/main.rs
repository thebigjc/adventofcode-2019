use png;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::iter;
use std::path::Path;

fn parse(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Bad number"),
    }
}

fn one(chunks: &Vec<&[u8]>) -> usize {
    let min_chunk = (*chunks.iter().min_by_key(|x| x.iter().filter(|a| **a == 0).count()).unwrap()).to_vec();

    let one = min_chunk.iter().filter(|x| **x == 1).count();
    let two = min_chunk.iter().filter(|x| **x == 2).count();
    one * two
}

fn two(layers: &Vec<&[u8]>, size: usize) -> Vec<u8> {
    let mut output: Vec<u8> = iter::repeat(2).take(size).collect();

    for l in layers {
        for (i, b) in (0..size).zip(l.iter()) {
            let mut a = output[i];
            if a == 2 {
                a = *b;
            }

            output[i] = a
        }
    }

    let out = output.iter().map(|x| *x).collect();
    out
}

fn main() {
    let nums: Vec<u8> = fs::read_to_string("day8.txt")
        .unwrap()
        .trim()
        .chars()
        .map(parse)
        .collect();
    let width = 25;
    let height = 6;

    let layers: Vec<&[u8]> = nums
        .chunks(width * height)
        .collect();

    let ret = one(&layers);
    println!("{}", ret);
    let path = Path::new(r"day8.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data = Vec::new();

    for x in two(&layers, width * height) {
        let d = match x {
            0 => [0, 0, 0, 255],
            1 => [255, 255, 255, 255],
            2 => [0, 0, 0, 0],
            _ => panic!("Unknown pixel value"),
        };
        data.extend(&d);
    }

    writer.write_image_data(&data).unwrap();
}
