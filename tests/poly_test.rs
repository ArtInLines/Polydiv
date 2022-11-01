use polydiv::*;

#[test]
fn new_poly() {
    assert_eq!(Polynomial::new().coefs, Vec::new());
    assert_eq!(Polynomial::with_capacity(10).coefs, Vec::new());
    assert_eq!(Polynomial::from_vec(vec![1, 2, 3]).coefs, vec![1, 2, 3]);
    assert_eq!(Polynomial::new(), Polynomial::with_capacity(100));
}

#[test]
fn display() {
    let p1 = Polynomial::from_vec(vec![1, 2, 3]);
    assert_eq!(format!("{p1}"), "3x^2 + 2x + 1");

    let p2 = Polynomial::from_vec(vec![7, 2, 0, 88, -3, 0]);
    assert_eq!(format!("{p2}"), "-3x^4 + 88x^3 + 2x + 7");
}
