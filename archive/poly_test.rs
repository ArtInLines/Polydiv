use num::{complex::Complex64, rational::Ratio, One, Zero};
use polydiv::*;

#[test]
fn new_poly() {
    assert_eq!(
        Polynomial::<Complex64>::new().coefs,
        Vec::<Complex64>::new()
    );
    assert_eq!(
        Polynomial::<Complex64>::with_capacity(10).coefs,
        Vec::<Complex64>::new()
    );
    assert_eq!(
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(1),
                Ratio::from_integer(2),
                Ratio::from_integer(3)
            ],
            0
        )
        .coefs,
        vec![
            Ratio::from_integer(1),
            Ratio::from_integer(2),
            Ratio::from_integer(3)
        ]
    );
    assert_eq!(
        Polynomial::<Complex64>::new(),
        Polynomial::<Complex64>::with_capacity(100)
    );

    assert_eq!(
        <Polynomial<Ratio<i32>> as Zero>::zero(),
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(0),
                Ratio::from_integer(0),
                Ratio::from_integer(0)
            ],
            15
        )
    );

    assert_eq!(
        <Polynomial<Ratio<i32>> as One>::one(),
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(0),
                Ratio::from_integer(0),
                Ratio::from_integer(1)
            ],
            -2
        )
    );

    // assert_eq!(
    //     Polynomial::from_vec(
    //         vec![
    //             Ratio::new(4, 2),
    //             Ratio::from_integer(0),
    //             Ratio::from_integer(4)
    //         ],
    //         -1
    //     ),
    //     Polynomial::from_vec(vec![Ratio::from_integer(2)], 1)
    // )
}

#[test]
fn display() {
    let p = Polynomial::<Ratio<i32>>::new();
    assert_eq!(format!("{p}"), "0");

    let p = Polynomial::from_vec(
        vec![
            Ratio::from_integer(1),
            Ratio::from_integer(2),
            Ratio::from_integer(3),
            Ratio::from_integer(4),
            Ratio::from_integer(5),
            Ratio::from_integer(6),
        ],
        -2,
    );
    assert_eq!(format!("{p}"), "6x^3 + 5x^2 + 4x + 3 + 2x^(-1) + x^(-2)");

    let p = Polynomial::from_vec(
        vec![
            Ratio::from_integer(7),
            Ratio::from_integer(2),
            Ratio::from_integer(0),
            Ratio::from_integer(88),
            Ratio::from_integer(-3),
            Ratio::from_integer(0),
        ],
        -1,
    );
    assert_eq!(format!("{p}"), "-3x^3 + 88x^2 + 2 + 7x^(-1)");
    assert_eq!(p.degree(), 3);

    let p = Polynomial::from_vec(vec![Ratio::from_integer(2), Ratio::from_integer(-37)], 5);
    assert_eq!(format!("{p}"), "-37x^6 + 2x^5");

    let p = Polynomial::from_vec(
        vec![
            Ratio::from_integer(2),
            Ratio::from_integer(-37),
            Ratio::from_integer(-4),
        ],
        -17,
    );
    // This is how negative numbers are supposed to look
    // See commented note in the fmt function
    assert_eq!(format!("{p}"), "-4x^(-15) + -37x^(-16) + 2x^(-17)");
    assert_eq!(p.degree(), -15);
}

#[test]
fn getters() {
    let mut p = Polynomial::from_vec(
        vec![
            Ratio::from_integer(1),
            Ratio::from_integer(2),
            Ratio::from_integer(3),
        ],
        -2,
    );
    assert_eq!(p.len(), 3);
    assert_eq!(*p.get(0).unwrap(), Ratio::from_integer(3));
    assert_eq!(*p.get_at(1).unwrap(), Ratio::from_integer(2));

    let coef = p.get_mut(-1).unwrap();
    *coef += 10;
    assert_eq!(*p.get(-1).unwrap(), Ratio::from_integer(12));

    *p.get_at_mut(0).unwrap() *= 10;
    assert_eq!(*p.get_at(0).unwrap(), Ratio::new(10, 1));
    assert_eq!(*p.get(-2).unwrap(), *p.get_at(0).unwrap());

    let p1 = Polynomial::from_vec(vec![Ratio::from_integer(3), Ratio::from_integer(5)], 1);
    let mut p2 = Polynomial::from_vec(
        vec![
            Ratio::from_integer(3),
            Ratio::from_integer(5),
            Ratio::from_integer(0),
        ],
        -1,
    );
    p2.shl(2);
    assert_eq!(p1, p2);
    p2.shr_lossles(1);
    assert_ne!(p1, p2);
}

#[test]
fn shift() {
    let mut p = Polynomial::from_vec(
        vec![
            Ratio::from_integer(1),
            Ratio::from_integer(2),
            Ratio::from_integer(3),
        ],
        0,
    );
    p.shl(4);
    *p.get_mut(3).unwrap() += 3;
    *p.get_mut(2).unwrap() *= 3;
    println!("{:?}", p.coefs);
    assert_eq!(*p.get(0).unwrap(), Ratio::from_integer(0));
    assert_eq!(*p.get(2).unwrap(), Ratio::from_integer(0));
    assert_eq!(*p.get(3).unwrap(), Ratio::from_integer(3));
    assert_eq!(p.len(), 7);
    p.shr_lossy(5);
    assert_eq!(p.len(), 2);
    assert_eq!(*p.get(0).unwrap(), Ratio::from_integer(2));
}

#[test]
fn ops() {
    let mut p1 = Polynomial::new();
    let mut p2 = Polynomial::with_capacity(10);
    let mut p3 = p1.simple_mul(Ratio::from_integer(2), 3);

    // Set
    p1.set(
        0,
        0,
        vec![
            Ratio::from_integer(3),
            Ratio::from_integer(5),
            Ratio::from_integer(0),
            Ratio::from_integer(7),
            Ratio::from_integer(0),
        ],
    );
    p1.normalize();
    assert_eq!(format!("{p1}"), "7x^3 + 5x + 3");
    assert_eq!(p1.degree(), 3);

    // Shift & simple mul
    assert_eq!(p2.len(), 0);
    p2.shl_with(2, Ratio::from_integer(1))
        .simple_mul_mut(Ratio::from_integer(3), 0);
    assert_eq!(p2.len(), 2);
    assert_eq!(format!("{p2}"), "3x + 3");
    assert_eq!(p2.init_pow(), 0);
    p2.simple_mul_mut(Ratio::from_integer(5), 2);
    assert_eq!(p2.init_pow(), 2);
    assert_eq!(p2.len(), 4);
    assert_eq!(format!("{p2}"), "15x^3 + 15x^2");

    // normalize
    assert_eq!(p3.normalize().len(), 0);
    assert_eq!(format!("{p3}"), "0");

    assert_eq!(format!("{p1}"), "7x^3 + 5x + 3");
    assert_eq!(format!("{p2}"), "15x^3 + 15x^2");
    // Add
    assert_eq!(
        format!("{}", p1.clone() + p2.clone()),
        "22x^3 + 15x^2 + 5x + 3"
    );

    // Sub
    assert_eq!(
        format!(
            "{}",
            p1.clone() + Polynomial::from_vec(vec![Ratio::from_integer(8)], 3) - p2.clone()
        ),
        "-15x^2 + 5x + 3"
    );

    // Mul
    assert_eq!(
        format!("{}", p1.clone() * p2.clone()),
        "105x^6 + 105x^5 + 75x^4 + 120x^3 + 45x^2"
    );
    assert_eq!(format!("{}", p1.clone() * p3.clone()), "0");
    assert_eq!(format!("{}", p3.clone() * p1.clone()), "0");

    // Div
    assert_eq!(
        format!("{}", p1.simple_div(Ratio::from_integer(2), 2)),
        "7/2x + 5/2x^(-1) + 3/2x^(-2)"
    );
    assert_eq!(format!("{}", p1.clone() / p1.clone()), "1");
}

#[test]
fn div1() {
    let p1 = Polynomial::from_vec(
        vec![
            Ratio::from_integer(15),
            Ratio::from_integer(8),
            Ratio::from_integer(1),
        ],
        0,
    );
    let p2 = Polynomial::from_vec(vec![Ratio::from_integer(5), Ratio::from_integer(1)], 0);
    let q = Polynomial::from_vec(vec![Ratio::from_integer(3), Ratio::from_integer(1)], 0);
    let r = Polynomial::zero();
    assert_eq!(p1.div_mod(&p2), (q, r));
}

#[test]
fn div2() {
    let p1 = Polynomial::from_vec(
        vec![
            Ratio::from_integer(8),
            Ratio::from_integer(10),
            Ratio::from_integer(7),
            Ratio::from_integer(2),
        ],
        0,
    );
    let p2 = Polynomial::from_vec(vec![Ratio::from_integer(2), Ratio::from_integer(1)], 0);
    let q = Polynomial::from_vec(
        vec![
            Ratio::from_integer(4),
            Ratio::from_integer(3),
            Ratio::from_integer(2),
        ],
        0,
    );
    let r = Polynomial::zero();
    assert_eq!(p1.div_mod(&p2), (q, r));
}

#[test]
fn div3() {
    let p1 = Polynomial::from_vec(
        vec![
            Ratio::from_integer(6),
            Ratio::from_integer(-3),
            Ratio::from_integer(1),
        ],
        0,
    );
    let p2 = Polynomial::from_vec(vec![Ratio::from_integer(2), Ratio::from_integer(1)], 0);
    let q = Polynomial::from_vec(vec![Ratio::from_integer(-5), Ratio::from_integer(1)], 0);
    let r = Polynomial::from_vec(vec![Ratio::from_integer(16)], 0);
    assert_eq!(p1.div_mod(&p2), (q, r));
}

#[test]
fn div4() {
    let p1 = Polynomial::from_vec(
        vec![
            Ratio::from_integer(-4),
            Ratio::from_integer(-2),
            Ratio::from_integer(4),
            Ratio::from_integer(2),
        ],
        0,
    );
    let p2 = Polynomial::from_vec(vec![Ratio::from_integer(-1), Ratio::from_integer(1)], 0);
    let q = Polynomial::from_vec(
        vec![
            Ratio::from_integer(4),
            Ratio::from_integer(6),
            Ratio::from_integer(2),
        ],
        0,
    );
    let r = Polynomial::zero();
    assert_eq!(p1.div_mod(&p2), (q, r));
}

#[test]
fn inv() {
    let polys = [
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(-4),
                Ratio::from_integer(-2),
                Ratio::from_integer(4),
                Ratio::from_integer(2),
            ],
            0,
        ),
        Polynomial::from_vec(vec![Ratio::from_integer(-1), Ratio::from_integer(1)], 0),
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(4),
                Ratio::from_integer(6),
                Ratio::from_integer(2),
            ],
            0,
        ),
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(6),
                Ratio::from_integer(-3),
                Ratio::from_integer(1),
            ],
            0,
        ),
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(15),
                Ratio::from_integer(8),
                Ratio::from_integer(1),
            ],
            0,
        ),
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(2),
                Ratio::from_integer(-37),
                Ratio::from_integer(-4),
            ],
            -17,
        ),
        Polynomial::from_vec(vec![Ratio::from_integer(2), Ratio::from_integer(-37)], 5),
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(7),
                Ratio::from_integer(2),
                Ratio::from_integer(0),
                Ratio::from_integer(88),
                Ratio::from_integer(-3),
                Ratio::from_integer(0),
            ],
            -1,
        ),
        Polynomial::from_vec(
            vec![
                Ratio::from_integer(1),
                Ratio::from_integer(2),
                Ratio::from_integer(3),
                Ratio::from_integer(4),
                Ratio::from_integer(5),
                Ratio::from_integer(6),
            ],
            -2,
        ),
    ];

    for p in polys {
        // let mi = Polynomial::one() / p.clone();
        let ai = Polynomial::zero() - p.clone();

        let (p, ai) = dbg!(p, ai);
        println!("{}  ->  {}", p, ai);

        // assert_eq!(mi * p.clone(), Polynomial::one());
        assert_eq!(*(ai + p.clone()).normalize(), Polynomial::one());
    }
}

#[test]
fn iter() {
    // TODO
}
