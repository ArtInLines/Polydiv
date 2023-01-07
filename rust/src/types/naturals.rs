use crate::traits::*;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Natural(u64);

impl Add for Natural {
    type Output = Natural;
    fn add(self, rhs: Self) -> Self::Output {
        Natural(self.0 + rhs.0)
    }
}

impl AddAssign for Natural {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul for Natural {
    type Output = Natural;
    fn mul(self, rhs: Self) -> Self::Output {
        Natural(self.0 * rhs.0)
    }
}

impl MulAssign for Natural {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Sub for Natural {
    type Output = Option<Natural>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 >= rhs.0 {
            Some(Natural(self.0 - rhs.0))
        } else {
            None
        }
    }
}

impl Iterator for Natural {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == u64::MAX {
            None
        } else {
            Some(Natural(self.0 + 1))
        }
    }
}

impl Set<Natural> for Natural {
    fn size(&self) -> SetSize {
        SetSize::CountablyInfinite
    }
}
