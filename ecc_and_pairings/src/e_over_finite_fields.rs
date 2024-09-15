use std::{
    ops::{Add, Mul, Neg, Sub},
    vec,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Fq(u8);

impl Fq {
    const Q: u8 = 11;

    fn new(n: i32) -> Self {
        Fq(((n % Self::Q as i32 + Self::Q as i32) % Self::Q as i32) as u8)
    }
}

impl Add for Fq {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Fq::new(self.0 as i32 + other.0 as i32)
    }
}

impl Sub for Fq {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Fq::new(self.0 as i32 - other.0 as i32)
    }
}

impl Mul for Fq {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Fq::new((self.0 as i32 * other.0 as i32) % Self::Q as i32)
    }
}

impl Neg for Fq {
    type Output = Self;
    fn neg(self) -> Self {
        Fq::new(-(self.0 as i32))
    }
}
#[derive(Clone, Debug)]
struct Polynomial {
    coefficients: Vec<Fq>,
}

impl Polynomial {
    fn new(coefficients: Vec<Fq>) -> Self {
        Polynomial { coefficients }
    }

    fn evaluate(&self, x: Fq) -> Fq {
        self.coefficients
            .iter()
            .rev()
            .fold(Fq::new(0), |acc, &coeff| acc * x + coeff)
    }
}

impl Sub for Polynomial {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let max_degree = self.coefficients.len().max(other.coefficients.len());
        let mut result = vec![Fq::new(0); max_degree];
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            result[i] = result[i] + coeff;
        }
        for (i, &coeff) in other.coefficients.iter().enumerate() {
            result[i] = result[i] - coeff;
        }
        Polynomial::new(result)
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: Fq,
    y: Fq,
}

struct EllipticCurve {
    a: Fq,
    b: Fq,
}

impl EllipticCurve {
    fn new(a: Fq, b: Fq) -> Self {
        EllipticCurve { a, b }
    }

    fn add(&self, p1: &Point, p2: &Point) -> Point {
        if p1.x == p2.x && p1.y == p2.y {
            self.double(p1)
        } else if p1.x == p2.x {
            Point {
                x: Fq::new(0),
                y: Fq::new(0),
            } // Point at infinity
        } else {
            let m = (p2.y - p1.y) * inverse_mod((p2.x - p1.x).0, Fq::Q);
            let x3 = m * m - p1.x - p2.x;
            let y3 = m * (p1.x - x3) - p1.y;
            Point { x: x3, y: y3 }
        }
    }

    fn double(&self, p: &Point) -> Point {
        let m = (Fq::new(3) * p.x * p.x + self.a) * inverse_mod((Fq::new(2) * p.y).0, Fq::Q);
        let x3 = m * m - Fq::new(2) * p.x;
        let y3 = m * (p.x - x3) - p.y;
        Point { x: x3, y: y3 }
    }
}

fn inverse_mod(a: u8, m: u8) -> Fq {
    for i in 1..m {
        if (a as u16 * i as u16) % m as u16 == 1 {
            return Fq::new(i as i32);
        }
    }
    panic!("Inverse doesn't exist");
}

fn roots(poly: &Polynomial) -> Vec<Fq> {
    (0..Fq::Q)
        .filter_map(|x| {
            let fx = poly.evaluate(Fq::new(x as i32));
            if fx == Fq::new(0) {
                Some(Fq::new(x as i32))
            } else {
                None
            }
        })
        .collect()
}

pub fn run() {
    let f = Polynomial::new(vec![Fq::new(0), Fq::new(-2), Fq::new(0), Fq::new(1)]); // x^3 - 2x
    let e = EllipticCurve::new(Fq::new(-2), Fq::new(0)); // E: y^2 = x^3 + ax + b

    let p = Point {
        x: Fq::new(5),
        y: Fq::new(7),
    };
    let q = Point {
        x: Fq::new(8),
        y: Fq::new(10),
    };

    let y = Polynomial::new(vec![Fq::new(2), Fq::new(1)]); // y = x + 2
    let y_squared = Polynomial::new(vec![Fq::new(4), Fq::new(4), Fq::new(1)]); // y^2 = x^2 + 4x + 4
    let roots_poly = y_squared - f;
    println!("Roots of y^2 - f where y = x + 2: {:?}", roots(&roots_poly));

    let r = e.add(&p, &q);
    println!(
        "P ({:?}, {:?}) + Q ({:?}, {:?}) = ({:?}, {:?})",
        p.x, p.y, q.x, q.y, r.x, r.y
    );
}
