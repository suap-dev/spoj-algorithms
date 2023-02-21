const SECONDS_IN_A_DAY: u32 = 24 * 60 * 60;

pub mod functions;

use std::io::stdin;
use functions::{get_boxes_needed, get_cookies_needed, get_gluttons_cookies, read_number};

// different implementations below
pub fn ob1() {
    let mut cookies_needed: u32;
    let mut gluttons_cookies; // .0 -> gluttons attending; .1 -> cookies per box

    let mut temp_str: String = String::new();

    let number_of_test_runs = read_number(&mut temp_str);
    for _ in 0..number_of_test_runs {
        gluttons_cookies = get_gluttons_cookies(&mut temp_str);
        cookies_needed = get_cookies_needed(&mut temp_str, gluttons_cookies.0);
        println!("{}", get_boxes_needed(cookies_needed, gluttons_cookies.1));
    }
}

// unstable in rustc 1.56
pub fn ob2() {
    let mut lines_iterator = stdin().lines();

    let number_of_tests: u32 = lines_iterator.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..number_of_tests {
        let two = lines_iterator.next().unwrap().unwrap();
        let mut two_iter = two.split_ascii_whitespace();
        let gluttons: u32 = two_iter.next().unwrap().parse().unwrap();
        let cookies_in_a_box: u32 = two_iter.next().unwrap().parse().unwrap();
        let mut cookies_needed = 0;

        for _ in 0..gluttons {
            let eating_time: u32 = lines_iterator.next().unwrap().unwrap().parse().unwrap();
            cookies_needed += SECONDS_IN_A_DAY / eating_time;
        }
        let boxes_needed = cookies_needed / cookies_in_a_box
            + u32::from(cookies_needed % cookies_in_a_box != 0);
        println!("{}", boxes_needed);
    }
}

// it works exactly as obzartuchy1()
pub fn ob3() {
    let mut line: String = String::new();

    // let number_of_tests: u32 = lines_iterator.next().unwrap().unwrap().parse().unwrap();
    stdin().read_line(&mut line).unwrap();
    let number_of_tests: u32 = line.trim().parse().unwrap();

    for _ in 0..number_of_tests {
        // let two = lines_iterator.next().unwrap().unwrap();
        line.clear();
        stdin().read_line(&mut line).unwrap();

        let mut two_iter = line.trim().split_ascii_whitespace();
        let gluttons: u32 = two_iter.next().unwrap().parse().unwrap();
        let cookies_in_a_box: u32 = two_iter.next().unwrap().parse().unwrap();
        let mut cookies_needed = 0;

        for _ in 0..gluttons {
            // let eating_time: u32 = lines_iterator.next().unwrap().unwrap().parse().unwrap();
            line.clear();
            stdin().read_line(&mut line).unwrap();
            let eating_time: u32 = line.trim().parse().unwrap();
            cookies_needed += SECONDS_IN_A_DAY / eating_time;
        }
        let boxes_needed = cookies_needed / cookies_in_a_box
            + u32::from(cookies_needed % cookies_in_a_box != 0);
        println!("{}", boxes_needed);
    }
}
