use std::io::{stdin, Read};

fn main() {
    let mut input_string: String = String::new();

    stdin().read_line(&mut input_string).expect("Incorrect input");
    let number_of_tests: u16 = input_string.trim().parse().expect("Incorrect number format");

    for _ in 0..number_of_tests {
        
    }
}
