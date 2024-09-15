use std::{
    ops::{Add, Mul, Neg, Sub},
    vec,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fq {
    pub value: u8, // The value in the field
    pub q: u8,     // The modulus Q
}

impl Fq {
    pub fn new(n: i32, q: u8) -> Self {
        Fq {
            value: ((n % q as i32 + q as i32) % q as i32) as u8,
            q,
        }
    }
}

impl Add for Fq {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Fq::new(self.value as i32 + other.value as i32, self.q)
    }
}

impl Sub for Fq {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Fq::new(self.value as i32 - other.value as i32, self.q)
    }
}

impl Mul for Fq {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Fq::new(
            (self.value as i32 * other.value as i32) % self.q as i32,
            self.q,
        )
    }
}

impl Neg for Fq {
    type Output = Self;
    fn neg(self) -> Self {
        Fq::new(-(self.value as i32), self.q)
    }
}
#[derive(Clone, Debug)]
struct Polynomial {
    coefficients: Vec<Fq>,
    q: u8,
}

impl Polynomial {
    fn new(coefficients: Vec<Fq>, q: u8) -> Self {
        Polynomial { coefficients, q }
    }

    fn evaluate(&self, x: Fq) -> Fq {
        self.coefficients
            .iter()
            .rev()
            .fold(Fq::new(0, self.q), |acc, &coeff| acc * x + coeff)
    }
}

impl Sub for Polynomial {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let max_degree = self.coefficients.len().max(other.coefficients.len());
        let mut result = vec![Fq::new(0, self.q); max_degree];
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            result[i] = result[i] + coeff;
        }
        for (i, &coeff) in other.coefficients.iter().enumerate() {
            result[i] = result[i] - coeff;
        }
        Polynomial::new(result, self.q)
    }
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: Fq,
    pub y: Fq,
}

pub struct EllipticCurve {
    pub a: Fq,
    pub b: Fq,
    pub q: u8,
}

impl EllipticCurve {
    pub fn new(a: Fq, b: Fq, q: u8) -> Self {
        EllipticCurve { a, b, q }
    }

    pub fn add(&self, p1: &Point, p2: &Point) -> Point {
        if p1.x == p2.x && p1.y == p2.y {
            self.double(p1)
        } else if p1.x == p2.x {
            Point {
                x: Fq::new(0, self.q),
                y: Fq::new(0, self.q),
            } // Point at infinity
        } else {
            let m = (p2.y - p1.y) * inverse_mod((p2.x - p1.x).value, self.q);
            let x3 = m * m - p1.x - p2.x;
            let y3 = m * (p1.x - x3) - p1.y;
            Point { x: x3, y: y3 }
        }
    }

    fn double(&self, p: &Point) -> Point {
        let m = (Fq::new(3, self.q) * p.x * p.x + self.a)
            * inverse_mod((Fq::new(2, self.q) * p.y).value, self.q);
        let x3 = m * m - Fq::new(2, self.q) * p.x;
        let y3 = m * (p.x - x3) - p.y;
        Point { x: x3, y: y3 }
    }
}

pub fn inverse_mod(a: u8, m: u8) -> Fq {
    for i in 1..m {
        if (a as u16 * i as u16) % m as u16 == 1 {
            return Fq::new(i as i32, m);
        }
    }
    panic!("Inverse doesn't exist");
}

fn roots(poly: &Polynomial, q: u8) -> Vec<Fq> {
    (0..q)
        .filter_map(|x| {
            let fx = poly.evaluate(Fq::new(x as i32, q));
            if fx == Fq::new(0, q) {
                Some(Fq::new(x as i32, q))
            } else {
                None
            }
        })
        .collect()
}

pub fn run() {
    let field_size = 11;
    let f = Polynomial::new(
        vec![
            Fq::new(0, field_size),
            Fq::new(-2, field_size),
            Fq::new(0, field_size),
            Fq::new(1, field_size),
        ],
        11,
    ); // x^3 - 2x
    let e = EllipticCurve::new(Fq::new(-2, field_size), Fq::new(0, field_size), field_size); // E: y^2 = x^3 + ax + b

    let p = Point {
        x: Fq::new(5, field_size),
        y: Fq::new(7, field_size),
    };
    let q = Point {
        x: Fq::new(8, field_size),
        y: Fq::new(10, field_size),
    };

    let y = Polynomial::new(
        vec![Fq::new(2, field_size), Fq::new(1, field_size)],
        field_size,
    ); // y = x + 2
    let y_squared = Polynomial::new(
        vec![
            Fq::new(4, field_size),
            Fq::new(4, field_size),
            Fq::new(1, field_size),
        ],
        field_size,
    ); // y^2 = x^2 + 4x + 4
    let roots_poly = y_squared - f;
    println!(
        "Roots of y^2 - f where y = x + 2: {:?}",
        roots(&roots_poly, field_size)
    );

    let r = e.add(&p, &q);
    println!(
        "P ({:?}, {:?}) + Q ({:?}, {:?}) = ({:?}, {:?})",
        p.x.value, p.y.value, q.x.value, q.y.value, r.x.value, r.y.value
    );
}
