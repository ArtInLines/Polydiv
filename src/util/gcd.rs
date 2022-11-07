use num::Num;

pub struct ExtendedGcd<A> {
    gcd: A,
    x: A,
    y: A,
}

pub fn extended_gcd<A>(a: A, b: A) -> ExtendedGcd<A>
where
    A: PartialOrd + Num + Copy,
{
    let mut a = a;
    let mut b = b;
    // Algorithm adapted from https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    if a < b {
        (a, b) = (b, a)
    }
    let mut r = a;
    let mut s = b;
    let mut x = A::one();
    let mut y = A::zero();

    while !s.is_zero() {
        let quot = r / s;
        (r, s) = (s, r - quot * s);
        (x, y) = (y, x - quot * y);
    }

    if !b.is_zero() {
        y = (r - x * a) / b;
    } else {
        y = A::zero();
    }

    ExtendedGcd { gcd: r, x, y }
}

pub fn gcd<A>(a: A, b: A) -> A
where
    A: Num + PartialOrd + Copy,
{
    let mut a = a;
    let mut b = b;
    if a < b {
        (a, b) = (b, a)
    }
    while !b.is_zero() {
        (a, b) = (b, a % b)
    }
    a
}
