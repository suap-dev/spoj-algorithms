use std::io::stdin;

fn main() {
    let mut temp_string: String = String::new();

    stdin()
        .read_line(&mut temp_string)
        .expect("Incorrect input");
    let number_of_tests: u16 = temp_string
        .trim()
        .parse()
        .expect("Incorrect number format");

    for _ in 0..number_of_tests {

        temp_string.clear();
        stdin()
            .read_line(&mut temp_string)
            .expect("Incorrect input");
        let number: u32 = temp_string
            .trim()
            .parse()
            .expect("Incorrect number format");

        print!("{} {}\n", change_base(number, 16), change_base(number, 11));
    }
}

#[inline]
fn change_base(mut number: u32, base: u32) -> String {
    const DIGITS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut new_number = String::new();

    while number / base > 0 || number % base > 0 {
        new_number.push(DIGITS[(number % base) as usize]);
        number = number / base;
    }

    return new_number.chars().rev().collect();
}
