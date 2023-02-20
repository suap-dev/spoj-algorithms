use std::io::stdin;

pub fn dzialka() {
    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();

    for _ in 0..line.trim().parse::<usize>().unwrap() {
        line.clear();
        stdin().read_line(&mut line).unwrap();

        let steps = line.trim().parse::<u32>().unwrap();
        println!("{}", steps * steps);
    }
}
