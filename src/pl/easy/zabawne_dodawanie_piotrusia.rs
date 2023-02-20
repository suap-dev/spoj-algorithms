use std::io;

pub fn piotrus() {
    let mut text: String = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Wrong iterations number!");

    let iterations = text
        .trim()
        .parse::<u8>()
        .expect("Number of iterations is not an integer!");

    for _ in 0..iterations {
        text.clear();
        io::stdin()
            .read_line(&mut text)
            .expect("Wrong number input!");

        let mut base = text.trim().to_string();
        let mut reversed: String = base.chars().rev().collect();

        let mut counter = 0;
        while base != reversed {
            base = (base.parse::<u32>().expect("Base is not an integer")
                + reversed.parse::<u32>().expect("Reversed is not an integer"))
            .to_string();
            reversed = base.chars().rev().collect();
            counter += 1;
        }
        println!("{} {}", base, counter);
    }
}
