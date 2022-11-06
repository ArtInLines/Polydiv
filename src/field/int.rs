use super::Field;
use derive_more::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};

#[derive(
    Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Add,
    AddAssign,
    Div,
    DivAssign,
    Sub,
    SubAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
    Not,
    Deref,
)]
pub struct IntField(pub isize);

impl PartialEq<isize> for IntField {
    fn eq(&self, other: &isize) -> bool {
        self.0 == *other
    }
}

impl PartialEq<IntField> for &IntField {
    fn eq(&self, other: &IntField) -> bool {
        self.0 == other.0
    }
}

impl Add<isize> for IntField {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        IntField(self.0 + rhs)
    }
}

impl AddAssign<isize> for IntField {
    fn add_assign(&mut self, rhs: isize) {
        self.0 += rhs
    }
}

impl Mul<isize> for IntField {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        IntField(self.0 * rhs)
    }
}

impl Mul for IntField {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        IntField(self.0 * rhs.0)
    }
}

impl MulAssign<isize> for IntField {
    fn mul_assign(&mut self, rhs: isize) {
        self.0 *= rhs
    }
}

impl MulAssign for IntField {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for IntField {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        IntField(self.0 / rhs.0)
    }
}

impl DivAssign for IntField {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl From<isize> for IntField {
    fn from(x: isize) -> Self {
        IntField(x)
    }
}

impl Neg for IntField {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Field for IntField {
    fn one_el() -> Self {
        IntField(1)
    }

    fn zero_el() -> Self {
        IntField(0)
    }

    fn div_mod(&self, rhs: &Self) -> (Self, Self) {
        (IntField(self.0 / rhs.0), IntField(self.0 % rhs.0))
    }
}
