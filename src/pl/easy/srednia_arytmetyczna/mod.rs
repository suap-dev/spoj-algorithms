pub mod fractions;

use std::fs::read;
use std::io::stdin;

macro_rules! readln {
    ($s:expr) => {
        $s.clear();
        stdin().read_line(&mut $s).unwrap();
    };
}
macro_rules! parse {
    ($s:expr, $type:ty) => {
        $s.trim().parse::<$type>().unwrap()
    };
}

use std::time::*;
pub fn srednia() {
    let mut line = String::new();

    readln!(line);
    let now = Instant::now();

    for _ in 0..parse!(line, usize) {
        readln!(line);
        let strings = line.split_ascii_whitespace();
        let mut numbers_u: Vec<u32> = Vec::new();
        for string in strings {
            numbers_u.push(parse!(string, u32));
        }
        let sum: u32 = numbers_u.iter().sum();
        let avg: f32 = sum as f32 / numbers_u.len() as f32;
        println!(
            "{}",
            numbers_u
                .iter()
                .min_by(|&&x, &&y| ((avg - x as f32).abs()).total_cmp(&((y as f32) - avg).abs()))
                .unwrap()
        );
    }

    let elapsed = now.elapsed();
    println!("\n1) -> {}\n", elapsed.as_nanos());
}

pub fn srednia2() {
    let mut line: String = String::new();

    readln!(line);
    let now = Instant::now();

    for _ in 0..parse!(line, usize) {
        readln!(line);
        let strings = line.split_ascii_whitespace();
        let mut numbers_u: Vec<u32> = Vec::new();
        for string in strings {
            numbers_u.push(parse!(string, u32));
        }

        let mut avg: f64 = 0f64;
        let mut amount = 0;
        for &number in &numbers_u {
            avg += number as f64;
            amount += 1;
        }
        avg /= amount as f64;

        let mut closest: u32 = 0;
        let mut distance = f64::MAX;
        for number in numbers_u {
            let new_distance = (number as f64 - avg).abs();
            if new_distance < distance {
                distance = new_distance;
                closest = number;
            } 
        }

        println!("{}", closest);
    }

    let elapsed = now.elapsed();
    println!("\n2) -> {}\n", elapsed.as_nanos());
}
