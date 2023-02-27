use std::io::{stdin, BufRead};

pub fn test() {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Reading failed");
    let t: usize = buf.trim().parse().expect("Parsing failed");
    for _ in 0..t {
        buf.clear();
        stdin.read_line(&mut buf).expect("Reading failed");
        println!("{}", col(buf.trim().parse().expect("Parsing failed")));
    }
}

const fn col(mut s: u32) -> u32 {
    let mut index: u32 = 0;
    while s != 1 {
        if s % 2 == 0 {
            s /= 2;
        } else {
            s = 3 * s + 1;
        }
        index += 1;
    }

    index
}
