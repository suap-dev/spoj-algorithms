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
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Reading failed");
}