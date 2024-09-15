use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Debug, PartialEq)]
struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    fn new(coefficients: Vec<f64>) -> Self {
        Polynomial { coefficients }
    }

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficients
            .iter()
            .rev()
            .fold(0.0, |acc, &coeff| acc * x + coeff)
    }

    fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }
}

impl Add for Polynomial {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let max_degree = self.degree().max(other.degree());
        let mut result = vec![0.0; max_degree + 1]; // constant
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            result[i] += &coeff;
        }

        for (i, &coeff) in other.coefficients.iter().enumerate() {
            result[i] += &coeff;
        }

        Polynomial::new(result)
    }
}

impl Sub for Polynomial {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let max_degree = self.degree().max(other.degree());
        let mut result = vec![0.0; max_degree + 1]; // constant
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            result[i] += &coeff;
        }

        for (i, &coeff) in other.coefficients.iter().enumerate() {
            result[i] -= &coeff;
        }

        Polynomial::new(result)
    }
}

impl Mul for Polynomial {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let max_degree = self.degree() + other.degree();
        let mut result = vec![0.0; max_degree + 1]; // constant
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            for (j, &coeff2) in other.coefficients.iter().enumerate() {
                result[i + j] += &coeff * &coeff2;
            }
        }

        Polynomial::new(result)
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

struct EllipticCurve<T> {
    a: T,
    b: T,
}

impl<T> EllipticCurve<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone,
{
    fn new(a: T, b: T) -> Self {
        EllipticCurve { a, b }
    }

    fn add(&self, p1: &Point, p2: &Point) -> Point {
        if p1.x == p2.x && p1.y == p2.y {
            self.double(p1)
        } else if p1.x == p2.x {
            Point {
                x: f64::INFINITY,
                y: f64::INFINITY,
            } // Point at infinity
        } else {
            let m = (p2.y - p1.y) / (p2.x - p1.x);
            let x3 = m * m - p1.x - p2.x;
            let y3 = m * (p1.x - x3) - p1.y;
            Point { x: x3, y: y3 }
        }
    }

    fn double(&self, p: &Point) -> Point {
        let fp = Polynomial::new(vec![-2.0, 0.0, 3.0 * p.x]);
        let m = fp.evaluate(p.x) / (2.0 * p.y);
        let x3 = m * m - 2.0 * p.x;
        let y3 = m * (p.x - x3) - p.y;
        Point { x: x3, y: y3 }
    }
}

fn roots(poly: &Polynomial) -> Vec<f64> {
    // This is a simplified root-finding method
    let mut roots = Vec::new();
    for x in -1000..1000 {
        let x = x as f64 / 100.0;
        if poly.evaluate(x).abs() < 1e-6 {
            roots.push(x);
        }
    }
    roots
}

pub fn run() {
    let f = Polynomial::new(vec![0.0, -2.0, 0.0, 1.0]); // x^3 - 2x
    let e = EllipticCurve::new(-2.0, 0.0);

    let p1 = Point { x: -1.0, y: -1.0 };
    let p2 = Point { x: 0.0, y: 0.0 };
    let p3 = Point { x: 2.0, y: 2.0 };

    println!("P1 + P2 = {:?}", e.add(&p1, &p2));
    println!("P2 + P3 = {:?}", e.add(&p2, &p3));
    println!("P1 + P3 = {:?}", e.add(&p1, &p3));
    println!("P2 + P2 = {:?}", e.add(&p2, &p2));

    let y = Polynomial::new(vec![-1.5, -0.5, 0.0]); // -1/2*x - 3/2
    let y_squared = y.clone() * y;
    let roots_poly = y_squared - f;
    println!("Roots of y^2 - f: {:?}", roots(&roots_poly));

    println!("2*P1 = {:?}", e.double(&p1));
}
