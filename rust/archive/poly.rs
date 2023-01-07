mod iter {
    use super::*;
    use num::Num;

    #[derive(Debug, Clone)]
    pub struct PolyCoefIter<T: Num + Copy + std::fmt::Display> {
        pub pow: isize,
        pub coef: T,
        pub current_idx: usize,
        pub poly: Polynomial<T>,
    }

    impl<T> Iterator for PolyCoefIter<T>
    where
        T: Num + Copy + std::fmt::Display,
    {
        type Item = Self;
        fn next(&mut self) -> Option<Self::Item> {
            if self.current_idx == 0 {
                None
            } else {
                self.current_idx -= 1;
                self.pow -= 1;
                if let Some(c) = self.poly.get_at(self.current_idx).copied() {
                    self.coef = c;
                    Some(self).cloned()
                } else {
                    None
                }
            }
        }
    }

    impl<T> IntoIterator for Polynomial<T>
    where
        T: Num + Copy + std::fmt::Display,
    {
        type IntoIter = PolyCoefIter<T>;
        type Item = <PolyCoefIter<T> as Iterator>::Item;
        fn into_iter(self) -> Self::IntoIter {
            PolyCoefIter {
                pow: self.degree(),
                coef: self.coefs.last().unwrap_or(&T::zero()).clone(),
                current_idx: if self.len() == 0 { 0 } else { self.len() - 1 },
                poly: self.clone(),
            }
        }
    }
}

pub use iter::*;
use num::{Num, One, Zero};
use std::{
    fmt::{Debug, Display},
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, RangeBounds, Rem, RemAssign, Sub,
        SubAssign,
    },
};

#[derive(Debug, Clone)]
pub struct Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    pub coefs: Vec<T>,
    pub zero_pow_idx: usize,
}

impl<T> Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    pub fn new() -> Self {
        Polynomial {
            coefs: Vec::new(),
            zero_pow_idx: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Polynomial {
            coefs: Vec::with_capacity(capacity),
            zero_pow_idx: 0,
        }
    }

    pub fn from_vec(vec: Vec<T>, init_pow: isize) -> Self {
        let mut p = Polynomial {
            coefs: vec,
            zero_pow_idx: 0,
        };

        if init_pow < 0 {
            p.zero_pow_idx = (-init_pow) as usize
        } else if init_pow > 0 {
            p.extend_right(init_pow as usize);
            p.zero_pow_idx = 0;
        }
        p.normalize();
        p
    }

    pub fn coefs(&self) -> &Vec<T> {
        &self.coefs
    }

    pub fn degree(&self) -> isize {
        (self.len() as isize) - (self.zero_pow_idx as isize) - 1
    }

    pub fn init_pow(&self) -> isize {
        if let Some((i, _)) = self.coefs.iter().enumerate().find(|(_, &c)| !c.is_zero()) {
            (i as isize) - (self.zero_pow_idx as isize)
        } else {
            0
        }
    }

    pub fn init_stored_pow(&self) -> isize {
        if self.zero_pow_idx > 0 {
            -(self.zero_pow_idx as isize)
        } else {
            0
        }
    }

    pub fn zero_pow_idx(&self) -> &usize {
        &self.zero_pow_idx
    }

    pub fn first(&self) -> Option<&T> {
        self.coefs.first()
    }

    pub fn last(&self) -> Option<&T> {
        self.coefs.last()
    }

    pub fn normalize(&mut self) -> &mut Self {
        // let min = if self.zero_pow_idx < self.len() {
        //     self.zero_pow_idx
        // } else if self.len() == 0 {
        //     0
        // } else {
        //     self.len() - 1
        // };

        // // Normalize ax^i + bx^(-i) to a/bx^0.
        // let mut x = T::zero();
        // for i in 0..min {
        //     let c1 = self.coefs[i];
        //     if c1.is_zero() {
        //         if let Some(c2) = self.get_at_mut(self.zero_pow_idx + i + 1) {
        //             x = x + *c2 / c1;
        //             *c2 = T::zero();
        //         }
        //     }
        //     *self.get_at_mut(i).unwrap() = T::zero();
        // }
        // match self.get_at_mut(self.zero_pow_idx) {
        //     Some(c) => *c = *c + x,
        //     None => {
        //         for _ in self.len()..self.zero_pow_idx + 1 {
        //             self.coefs.push(T::zero());
        //         }
        //         *self.coefs.get_mut(self.zero_pow_idx).unwrap() = x;
        //     }
        // }

        // Remove 0s at head
        while self.len() >= 1 {
            if let Some(&coef) = self.coefs.last() {
                if coef.is_zero() {
                    self.coefs.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Remove 0s at tail
        while self.zero_pow_idx > 0 {
            if let Some(&coef) = self.coefs.first() {
                if coef.is_zero() {
                    self.coefs.remove(0);
                    self.zero_pow_idx -= 1;
                } else {
                    break;
                }
            } else {
                self.zero_pow_idx = 0;
                break;
            }
        }
        self
    }

    pub fn len(&self) -> usize {
        self.coefs.len()
    }

    pub fn get(&self, idx: isize) -> Option<&T> {
        if idx >= 0 {
            self.coefs.get(idx as usize + self.zero_pow_idx)
        } else if idx < -(self.zero_pow_idx as isize) {
            None
        } else {
            self.coefs.get(self.zero_pow_idx - (-idx as usize))
        }
    }

    pub fn get_mut(&mut self, idx: isize) -> Option<&mut T> {
        if idx >= 0 {
            self.coefs.get_mut(idx as usize + self.zero_pow_idx)
        } else if idx < -(self.zero_pow_idx as isize) {
            None
        } else {
            self.coefs.get_mut(self.zero_pow_idx - (-idx as usize))
        }
    }

    pub fn get_at(&self, idx: usize) -> Option<&T> {
        self.coefs.get(idx)
    }

    pub fn get_at_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.coefs.get_mut(idx)
    }

    pub fn set_at(
        &mut self,
        range: impl RangeBounds<usize>,
        replace_with: impl IntoIterator<Item = T>,
    ) {
        self.coefs.splice(range, replace_with);
    }

    pub fn set(
        &mut self,
        range_start: isize,
        range_end: isize,
        replace_with: impl IntoIterator<Item = T>,
    ) {
        let mut range = (range_start, range_end);
        if range.0 + (self.zero_pow_idx as isize) < 0 {
            range.0 = 0;
        } else {
            range.0 += self.zero_pow_idx as isize;
        }
        if range.1 + (self.zero_pow_idx as isize) < 0 {
            range.1 = 0;
        } else if range.1 + (self.zero_pow_idx as isize) > self.len() as isize {
            range.1 = self.len() as isize
        } else {
            range.1 += self.zero_pow_idx as isize;
        }
        let range = (range.0 as usize)..(range.1 as usize);
        self.set_at(range, replace_with);
    }

    pub fn insert(&mut self, index: usize, elements: impl IntoIterator<Item = T>) {
        self.set_at(index..index, elements)
    }

    pub fn insert_coef(&mut self, index: usize, element: T) {
        self.insert(index, vec![element])
    }

    pub fn shr_lossy(&mut self, amount: usize) -> &mut Self {
        let n = if self.coefs.len() < amount {
            self.coefs.len()
        } else {
            amount
        };
        self.coefs.drain(0..n);
        if n > self.zero_pow_idx {
            self.zero_pow_idx = 0;
        } else {
            self.zero_pow_idx -= n;
        }
        self.normalize();
        self
    }

    pub fn shr_lossles(&mut self, amount: usize) -> &mut Self {
        self.zero_pow_idx += amount;
        self
    }

    pub fn shl_with(&mut self, amount: usize, el: T) -> &mut Self {
        let v = vec![el; amount];
        self.coefs.splice(0..0, v);
        self
    }

    pub fn shl(&mut self, amount: usize) -> &mut Self {
        self.shl_with(amount, T::zero())
    }

    pub fn extend_right(&mut self, amount: usize) -> &mut Self {
        self.shl(amount);
        self.zero_pow_idx += amount;
        self
    }

    pub fn extend_left(&mut self, amount: usize) -> &mut Self {
        self.coefs
            .splice(self.len()..self.len(), vec![T::zero(); amount]);
        self
    }

    pub fn simple_mul_mut(&mut self, coef: T, power: isize) -> &mut Self {
        if power < 0 {
            return self.simple_div_mut(coef, -power);
        }
        self.shl(power as usize);
        if coef != T::one() {
            self.coefs.iter_mut().for_each(|x| *x = *x * coef);
        }
        self
    }

    pub fn simple_mul(&self, coef: T, power: isize) -> Self {
        let mut c = self.clone();
        c.simple_mul_mut(coef, power);
        c
    }

    pub fn simple_div_mut(&mut self, coef: T, power: isize) -> &mut Self {
        if power < 0 {
            return self.simple_mul_mut(coef, -power);
        }
        if coef.is_zero() {
            panic!("Divide by zero error")
        }
        self.shr_lossles(power as usize);
        if coef != T::one() {
            self.coefs.iter_mut().for_each(|x| *x = *x / coef);
        }
        self
    }

    pub fn simple_div(&self, coef: T, power: isize) -> Self {
        let mut c = self.clone();
        c.simple_div_mut(coef, power);
        c
    }

    pub fn align_size_to(&mut self, other: &Self) -> &mut Self {
        if self.zero_pow_idx < other.zero_pow_idx {
            self.extend_right(other.zero_pow_idx - self.zero_pow_idx);
        }
        if self.len() < other.len() {
            self.extend_left(other.len() - self.len());
        }
        self
    }
}

impl<T> PartialEq for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn eq(&self, other: &Self) -> bool {
        if self.degree() != other.degree() {
            false
        } else {
            let mut c = self.clone();
            c.align_size_to(other);
            c.into_iter()
                .all(|item| match other.get_at(item.current_idx) {
                    Some(&c) => c == item.coef,
                    None => false,
                })
        }
    }
}

impl<T> Display for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = self.clone();
        // let mut c = dbg!(c);
        c.normalize();
        // let c = dbg!(c);
        let init_pow = c.init_stored_pow();
        let len = c.len();
        if len == 0 {
            return write!(f, "{}", T::zero());
        }

        let s = c
            .coefs
            .iter()
            .rev()
            .enumerate()
            .fold(String::new(), |acc, (idx, coef)| {
                // Note:
                // This formatting function doesn't correctly show negative numbers
                // instead it shows (+ -n)
                // however, we could only test for whether a number is negative
                // iff the field is ordered
                // which we don't want to assume here
                // Since this code will probably only be used by me in regard to fields of the characteristic 2 where negative numbers are the same as positive numbers, this shouldn't be an issue anyways

                let current_pow = init_pow + ((len - idx) as isize) - 1;
                let show_plus = !coef.is_zero() && idx != 0;
                let show_x = current_pow != 0 && !coef.is_zero();
                let show_pow = show_x && current_pow != 1;
                let show_coef = !coef.is_zero() && (*coef != T::one() || !show_x);

                let mut s = format!("{acc}");
                if show_plus {
                    s.push_str(&format!(" + "));
                }
                if show_coef {
                    s.push_str(&format!("{coef}"))
                }
                if show_x {
                    s.push('x');
                }
                if show_pow {
                    if current_pow < 0 {
                        s.push_str(&format!("^({})", current_pow))
                    } else {
                        s.push_str(&format!("^{}", current_pow))
                    }
                }
                s
            });
        write!(f, "{s}")
    }
}

impl<T> AddAssign for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn add_assign(&mut self, rhs: Self) {
        self.align_size_to(&rhs);
        self.coefs.iter_mut().enumerate().for_each(|(i, c1)| {
            if let Some(&c2) = rhs.get((i as isize) - (self.zero_pow_idx as isize)) {
                *c1 = *c1 + c2;
            }
        });
        self.normalize();
    }
}

impl<T> Add for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        c += rhs;
        c
    }
}

impl<T> SubAssign for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.align_size_to(&rhs);
        self.coefs.iter_mut().enumerate().for_each(|(i, c1)| {
            if let Some(&c2) = rhs.get((i as isize) - (self.zero_pow_idx as isize)) {
                *c1 = *c1 - c2;
            }
        });
        self.normalize();
    }
}

impl<T> Sub for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        c.sub_assign(rhs);
        c
    }
}

impl<T> Neg for Polynomial<T>
where
    T: Num + Copy + Neg<Output = T> + std::fmt::Display,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut c = self.clone();
        c.normalize();
        c.coefs.iter_mut().for_each(|c| *c = c.neg());
        c
    }
}

impl<T> MulAssign for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn mul_assign(&mut self, rhs: Self) {
        if rhs.len() == 0 {
            self.coefs.clear();
        } else {
            let zero_pow_idx = rhs.zero_pow_idx as isize;
            *self = rhs
                .coefs
                .clone()
                .iter_mut()
                .enumerate()
                .map(|(i, coef)| self.simple_mul(*coef, (i as isize) - zero_pow_idx))
                .reduce(|accum, item| accum + item)
                .expect("Error when multiplying two polynomials");
            self.normalize();
        }
    }
}

impl<T> Mul for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        c *= rhs;
        c
    }
}

impl<T> Zero for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn zero() -> Self {
        Self::from_vec(vec![T::zero()], 0)
    }

    fn is_zero(&self) -> bool {
        for c in self.coefs.clone() {
            if !c.is_zero() {
                return true;
            }
        }
        false
    }
}

impl<T> One for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn one() -> Self {
        Self::from_vec(vec![T::one()], 0)
    }
}

impl<T> RemAssign for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.div_mod(&rhs).1;
    }
}

impl<T> Rem for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        c %= rhs;
        c
    }
}

pub enum ParsePolyErr {
    ParseErr,
}

impl<T> Num for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    type FromStrRadixErr = ParsePolyErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Err(ParsePolyErr::ParseErr)
    }
}

impl<T> Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    pub fn div_mod(&self, rhs: &Self) -> (Self, Self) {
        // Algorithm taken and adapted from here: https://stackoverflow.com/a/26178457/13764271

        let mut num = self.clone();
        let mut den = rhs.clone();
        num.normalize();
        den.normalize();

        if num.degree() < den.degree() {
            return (Self::zero(), num);
        }
        let iterations = (num.degree() - den.degree()) as usize + 1;

        let mut quot = Polynomial::with_capacity(((num.degree() - den.degree()) as usize) + 1);
        let divisor = den.last().unwrap().clone();
        for _ in 0..iterations {
            let mult = *num.last().unwrap() / divisor;
            println!(
                "quot: {}  -  num: {}  -  den: {}  -  mult: {}",
                quot, num, den, mult
            );
            quot.insert_coef(0, mult);

            let deg_diff = num.degree() - den.degree();
            if !mult.is_zero() {
                let d = den.simple_mul(mult, deg_diff);
                println!("d: {}", d);
                num -= d;
            }
        }
        println!("quot: {}  -  num: {}  -  den: {}", quot, num, den);

        (quot, num)
    }

    // fn multiplicative_inv(&self) -> Option<Self> {
    //     if self.is_zero() {
    //         None
    //     } else {
    //         let mut rev_coefs = self.coefs.clone();
    //         rev_coefs.reverse();
    //         rev_coefs
    //             .iter_mut()
    //             .for_each(|c| *c = c.multiplicative_inv().unwrap_or(T::zero()));
    //         Some(Polynomial {
    //             coefs: rev_coefs,
    //             zero_pow_idx: self.len() - 1 + self.zero_pow_idx,
    //         })
    //     }
    // }
}

impl<T> DivAssign for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    fn div_assign(&mut self, rhs: Self) {
        let (q, _) = self.div_mod(&rhs);
        *self = q;
    }
}

impl<T> Div for Polynomial<T>
where
    T: Num + Copy + std::fmt::Display,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        c /= rhs;
        c
    }
}
