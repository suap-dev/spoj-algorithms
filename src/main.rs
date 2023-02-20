pub mod pl;

use std::io::stdin;
use pl::easy::srednia_arytmetyczna::*;
use pl::easy::srednia_arytmetyczna::fractions::*;

fn main() {
    let f1 = Fraction::new(1,3);
    let f2 = Fraction::new(1,2);

    println!("{}", f1 == f2);
}










/*

16
4 1 2 3 4 
4 4 3 2 1
4 0 3 2 4
2 3 6 12 5
389 9320 189 405 289 40
590 403 392 950 2093 18 5093
2 3
50 3 290 100 100 100 2
4 1 2 3 4 
4 4 3 2 1
4 0 3 2 4
2 3 6 12 5
389 9320 189 405 289 40
590 403 392 950 2093 18 5093
2 3
50 3 290 100 100 100 2

*/