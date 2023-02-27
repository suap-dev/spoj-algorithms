#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

mod pl;

use std::io::{stdin, BufRead};

fn main() {
    test();
}

fn test() {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();

    // number of tests:
    stdin.read_line(&mut buf).expect("Reading failed");
    let t: usize = buf.trim().parse().unwrap();

    for _ in 0..t {
        buf.clear();
        stdin.read_line(&mut buf).expect("Reading failed");

        // TODO
    }
}
