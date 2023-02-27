use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{stdin, BufRead};

pub fn test() {
    let _stdin = stdin();
    let mut stdin = _stdin.lock();
    let mut buf: String = String::new();

    let mut map: SimplerMap = SimplerMap::create();

    stdin.read_line(&mut buf).unwrap();
    let maps = buf.trim().parse().unwrap();

    for _ in 0..maps {
        map.reset();

        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let clue_sets: u32 = buf.trim().parse().unwrap();
        for _ in 0..clue_sets {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let mut clues = buf.split_whitespace();
            map.go(
                clues.next().unwrap().trim().parse().unwrap(),
                clues.next().unwrap().trim().parse().unwrap(),
            );
        }
        map.print();
    }
}

struct SimplerMap {
    x: i32,
    y: i32,
}
impl SimplerMap {
    pub const fn create() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
    pub fn go(&mut self, x: i8, y: i32) {
        match x % 4 {
            0 => self.y += y, // north
            1 => self.y -= y, // south
            2 => self.x -= y, // west
            3 => self.x += y, // east
            _other => panic!("how?"),
        };
    }
    pub fn print(&self) {
        if self.x == 0 {
            self.print_y();
        } else if self.y == 0 {
            self.print_x();
        } else {
            self.print_y();
            self.print_x();
        }
    }
    fn print_y(&self) {
        match self.y.cmp(&0) {
            Greater => println!("0 {}", self.y),
            Less => println!("1 {}", -self.y),
            Equal => println!("studnia"),
        };
    }
    fn print_x(&self) {
        match self.x.cmp(&0) {
            Less => println!("2 {}", -self.x),
            Greater => println!("3 {}", self.x),
            Equal => println!("studnia"),
        };
    }
}
