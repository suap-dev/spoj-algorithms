use std::cmp::Ordering;
use std::cmp::Ordering::*;

#[derive(Eq)]
pub struct Fraction {
    numerator: u32,
    denominator: u32,
}
impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }
}
impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}
impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.numerator.cmp(&other.numerator)  self.denominator.cmp(&other.denominator)
        if self != other {
            if self.numerator < other.numerator {
                Less
            } else {
                Greater
            }
        } else {
            Equal
        }
    }
}
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
