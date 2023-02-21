use std::io::{stdin, stdout, BufRead, Write};

pub fn poz() {
    // we know the nunbers will be at most 1_000_000 digits long
    let mut buffer = String::with_capacity(1_000_010);

    // we get a locked sdin
    let stdin = stdin();
    let mut stdin = stdin.lock(); // spoj doesn't like stdin().lock()

    let stdout = stdout();
    let mut stdout = stdout.lock();

    stdin.read_line(&mut buffer).expect("Incorrect input");
    let number_of_tests: u16 = buffer.trim().parse().expect("Incorrect number format");

    for _ in 0..number_of_tests {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Incorrect input");
        let number: u32 = buffer.trim().parse().expect("Incorrect number format");

        // we can just format the number to hex using the :X identifier
        write!(stdout, "{:X} ", number).unwrap();
        writeln!(stdout, "{}", change_base(&mut buffer, number, 11)).unwrap();
        // println!();
    }
}

// it doesn't really matter which function we use
// recursive and iterative takes the same time
// also passing a string or even reversing and collecting doesn't impact
// the execution time.

#[inline]
fn change_base(buffer: &mut String, mut number: u32, base: u32) -> String {
    const DIGITS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    // let mut new_number: String = String::new();
    buffer.clear();

    while number / base > 0 || number % base > 0 {
        buffer.push(DIGITS[(number % base) as usize]);
        // print!("{}", DIGITS[(number % base) as usize]);
        number /= base;
    }

    buffer.chars().rev().collect()
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
