const SECONDS_IN_A_DAY: u32 = 24 * 60 * 60;

use std::io::stdin;

#[inline]
pub fn get_boxes_needed(cookies_needed: u32, cookies_per_box: u32) -> u32 {
    cookies_needed / cookies_per_box + u32::from(cookies_needed % cookies_per_box != 0)
}

#[inline]
pub fn get_cookies_needed(temp_str: &mut String, gluttons: u32) -> u32 {
    let mut cookies_needed = 0;

    for _ in 0..gluttons as usize {
        cookies_needed += SECONDS_IN_A_DAY / read_number(temp_str);
    }

    cookies_needed
}

#[inline]
pub fn get_gluttons_cookies(temp_str: &mut String) -> (u32, u32) {
    read_line(temp_str);
    let mut iter = temp_str.split_ascii_whitespace();
    (
        iter.next()
            .expect("Iterator empty")
            .parse()
            .expect("Not a valid u16 number"),
        iter.next()
            .expect("Iterator empty")
            .parse()
            .expect("Not a valid u16 number"),
    )
}

#[inline]
pub fn read_number(temp_str: &mut String) -> u32 {
    temp_str.clear();
    stdin().read_line(temp_str).expect("Not a valid input");
    temp_str.trim().parse().expect("Not a valid u32 number")
}

#[inline]
pub fn read_line(temp_str: &mut String) {
    temp_str.clear();
    stdin().read_line(temp_str).expect("Not a valid input");
}
