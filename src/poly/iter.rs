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
