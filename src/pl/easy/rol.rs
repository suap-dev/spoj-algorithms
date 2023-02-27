use std::io::{stdin, BufRead};

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

        let mut numbers = buf.split_whitespace().skip(1);
        let first = numbers.next().unwrap();
        for number in numbers {
            print!("{} ", number);
        }
        println!("{}", first);
    }
}
