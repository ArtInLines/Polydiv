use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

mod poly;
pub use poly::*;

pub struct AdditiveIter<F: Field> {
    item: F,
}

impl<F: Field> Iterator for AdditiveIter<F> {
    type Item = <F as Add>::Output;
    fn next(&mut self) -> Option<Self::Item> {
        if self.item == F::zero_el() {
            None
        } else {
            Some(self.item.clone() + self.item.clone())
        }
    }
}

pub struct MultiplicativeIter<F: Field> {
    item: F,
}

impl<F: Field> Iterator for MultiplicativeIter<F> {
    type Item = <F as Mul>::Output;
    fn next(&mut self) -> Option<Self::Item> {
        if self.item == F::one_el() {
            None
        } else {
            Some(self.item.clone() * self.item.clone())
        }
    }
}

pub trait Field:
    Add<Output = Self>
    + AddAssign
    + Neg<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Clone
    + PartialEq
    + Display
{
    fn zero_el() -> Self;

    fn one_el() -> Self;

    fn div_mod(&self, rhs: &Self) -> (Self, Self);

    fn div(&self, rhs: &Self) -> Self {
        self.div_mod(rhs).0
    }

    fn r#mod(&self, rhs: &Self) -> Self {
        self.div_mod(rhs).1
    }

    fn additive_iter(self) -> AdditiveIter<Self> {
        AdditiveIter { item: self }
    }

    fn multiplicative_iter(self) -> MultiplicativeIter<Self> {
        MultiplicativeIter { item: self }
    }

    fn additive_inv(&self) -> Self {
        self.clone().neg()
    }

    fn multiplicative_inv(&self) -> Option<Self> {
        if self.clone() == Self::zero_el() {
            None
        } else {
            Some(Field::div(&Self::one_el(), self))
        }
    }
}

pub trait FiniteField: Field {
    fn order_el() -> Self;
}
