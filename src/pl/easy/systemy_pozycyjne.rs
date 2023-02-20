use std::io::{stdin, BufRead};

pub fn poz() {
    // we know the nunbers will be at most 1_000_000 digits long
    let mut buffer = String::with_capacity(1_000_010);

    // we get a locked sdin
    let t_stdin = stdin();
    let mut stdin = t_stdin.lock(); // spoj doesn't like stdin().lock()

    stdin.read_line(&mut buffer).expect("Incorrect input");
    let number_of_tests: u16 = buffer.trim().parse().expect("Incorrect number format");

    for _ in 0..number_of_tests {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Incorrect input");
        let number: u32 = buffer.trim().parse().expect("Incorrect number format");
    
        // we can just format the number to hex using the :X identifier
        print!("{:X} ", number);
        print_changed(number, 11);
        println!();
    }
}

// it doesn't really matter which function we use
// recursive and iterative takes the same time
// also passing a string or even reversing and collecting doesn't impact
// the execution time.

#[inline]
fn change_base(mut number: u32, base: u32) -> String {
    let mut new_number: String = String::new();
    const DIGITS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    while number / base > 0 || number % base > 0 {
        new_number.push(DIGITS[(number % base) as usize]);
        // print!("{}", DIGITS[(number % base) as usize]);
        number = number / base;
    }

    new_number.chars().rev().collect()
}

fn print_changed(number: u32, base: u32) {
    const DIGITS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    if number > 0 || number % base > 0 {
        print_changed(number / base, base);
    }
    if number > 0 {
        print!("{}", DIGITS[(number % base) as usize]);
    }
}
