mod gcd;
pub use gcd::*;
use std::ops::Sub;

pub enum SetSize {
    Infinite,
    Finite(usize),
}

pub trait Normal {
    fn normalize(&mut self) -> &mut Self;
}

pub fn abs_dist<T>(x: &T, y: &T) -> T
where
    T: PartialOrd + Sub<Output = T> + Copy,
{
    if x >= y {
        *x - *y
    } else {
        *y - *x
    }
}
