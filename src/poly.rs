use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub coefs: Vec<isize>,
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial { coefs: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Polynomial {
            coefs: Vec::with_capacity(capacity),
        }
    }

    pub fn from_vec(vec: Vec<isize>) -> Self {
        Polynomial { coefs: vec }
    }

    pub fn normalize(&mut self) {
        while self.len() >= 1 {
            if let Some(&coef) = self.coefs.last() {
                if coef == 0 {
                    self.coefs.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.coefs.len()
    }

    pub fn get(&self, idx: usize) -> Option<&isize> {
        self.coefs.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut isize> {
        self.coefs.get_mut(idx)
    }

    pub fn shr(&mut self, amount: usize) {
        self.coefs.drain(0..amount);
    }

    pub fn shl(&mut self, amount: usize) {
        self.coefs.append(&mut vec![0; amount])
    }

    pub fn simple_mul_mut(&mut self, coef: isize, power: usize) {
        self.shl(power);
        if coef != 1 {
            self.coefs.iter_mut().for_each(|x| *x *= coef);
        }
    }

    pub fn simple_mul(self, coef: isize, power: usize) -> Self {
        let mut c = self.clone();
        c.simple_mul_mut(coef, power);
        c
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = self.clone();
        c.normalize();
        let l = c.coefs.len();
        let s = c
            .coefs
            .iter()
            .rev()
            .enumerate()
            .fold(String::new(), |acc, (idx, coef)| {
                if *coef == 0 {
                    format!("{acc}")
                } else if idx == 0 {
                    format!("{coef}x^{}", l - idx - 1)
                } else if idx == l - 1 {
                    format!("{acc} + {coef}")
                } else if idx == l - 2 {
                    format!("{acc} + {coef}x")
                } else {
                    format!("{acc} + {coef}x^{}", l - idx - 1)
                }
            });
        write!(f, "{s}")
    }
}

impl AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Self) {
        let self_len = self.len();
        let rhs_len = rhs.len();
        for i in 0..self_len {
            match rhs.get(i) {
                Some(&coef) => self.coefs[i] += coef,
                None => return,
            };
        }
        for i in self_len..rhs_len {
            match rhs.get(i) {
                Some(&coef) => self.coefs.push(coef),
                None => return,
            }
        }
        self.normalize();
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        c += rhs;
        c
    }
}

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        *self = rhs
            .coefs
            .clone()
            .iter_mut()
            .enumerate()
            .map(|(i, coef)| self.clone().simple_mul(*coef, i))
            .reduce(|accum, item| accum + item)
            .expect("Error when multiplying two polynomials");
        self.normalize();
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        c *= rhs;
        c
    }
}
