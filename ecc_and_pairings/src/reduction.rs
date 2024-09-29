use num_bigint::{BigUint, RandBigInt};
use num_traits::ToPrimitive;
use num_traits::{One, Zero};
use rand::Rng;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, PartialEq)]
struct FiniteField {
    prime: BigUint,
}

#[derive(Clone, Debug, PartialEq)]
struct FieldElement {
    value: BigUint,
    field: FiniteField,
}

#[derive(Clone, Debug, PartialEq)]
struct EllipticCurve {
    a: FieldElement,
    b: FieldElement,
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    curve: EllipticCurve,
}

#[derive(Clone, Debug)]
struct Divisor {
    points: Vec<(Point, i32)>,
}

impl FiniteField {
    fn new(prime: BigUint) -> Self {
        FiniteField { prime }
    }

    fn random_element(&self) -> FieldElement {
        let mut rng = rand::thread_rng();
        FieldElement {
            value: rng.gen_biguint_below(&self.prime),
            field: self.clone(),
        }
    }
}

impl FieldElement {
    fn new(value: BigUint, field: FiniteField) -> Self {
        let mut val = value % &field.prime;
        if val < BigUint::zero() {
            val += &field.prime;
        }
        FieldElement { value: val, field }
    }

    fn pow(&self, exp: &BigUint) -> Self {
        let mut base = self.clone();
        let mut result = FieldElement::new(BigUint::one(), self.field.clone());
        let mut exp = exp.clone();

        while exp > BigUint::zero() {
            if &exp % BigUint::from(2u32) == BigUint::one() {
                result = result * base.clone();
            }
            base = base.clone() * base;
            exp /= 2u32;
        }
        result
    }

    fn inv(&self) -> Self {
        // This implementation is based on Fermat's Little Theorem, which states that for a prime p and any integer a not divisible by p:
        // a^(p-1) ≡ 1 (mod p)
        // From this, we can derive that a^(p-2) is the multiplicative inverse of a in the field of integers modulo p.
        let exp = &self.field.prime - BigUint::from(2u32);
        self.pow(&exp)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.field, other.field);
        FieldElement::new(self.value + other.value, self.field)
    }
}

// Implement Sub for FieldElement
impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.field, other.field);
        let mut result = self.value - other.value;
        if result < BigUint::zero() {
            result += &self.field.prime;
        }
        FieldElement::new(result, self.field)
    }
}

// Implement Mul for FieldElement
impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.field, other.field);
        FieldElement::new(self.value * other.value, self.field)
    }
}

// Implement Div for FieldElement
impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.field, other.field);
        let inv = other.inv();
        self * inv
    }
}

impl EllipticCurve {
    fn new(a: FieldElement, b: FieldElement) -> Self {
        assert_eq!(a.field, b.field);
        EllipticCurve { a, b }
    }

    fn contains(&self, point: &Point) -> bool {
        if let (Some(x), Some(y)) = (point.x.clone(), point.y.clone()) {
            let lhs = y.clone() * y;
            let rhs = ((x.clone() * x.clone() * x.clone()) + (self.a.clone() * x)) + self.b.clone();
            lhs == rhs
        } else {
            true // Point at infinity is always on the curve
        }
    }

    fn random_point(&self) -> Point {
        loop {
            let x = self.a.field.random_element();
            let y_squared = ((x.clone() * x.clone() * x.clone()) + (self.a.clone() * x.clone()))
                + self.b.clone();
            if let Some(y) = self.sqrt(&y_squared) {
                return Point {
                    x: Some(x),
                    y: Some(y),
                    curve: self.clone(),
                };
            }
        }
    }

    fn sqrt(&self, a: &FieldElement) -> Option<FieldElement> {
        // For a general solution, implement Tonelli-Shanks algorithm
        let exp = (&a.field.prime + BigUint::one()) / BigUint::from(4u32);
        let root = a.pow(&exp);
        if &(root.clone() * root.clone()) == a {
            Some(root)
        } else {
            None
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        assert_eq!(self.curve, other.curve);

        if self.x.is_none() {
            return other.clone();
        }
        if other.x.is_none() {
            return self.clone();
        }

        let (x1, y1) = (self.x.as_ref().unwrap(), self.y.as_ref().unwrap());
        let (x2, y2) = (other.x.as_ref().unwrap(), other.y.as_ref().unwrap());

        if x1 == x2 && y1 != y2 {
            return Point {
                x: None,
                y: None,
                curve: self.curve.clone(),
            }; // Point at infinity
        }

        let m = if x1 == x2 {
            // Point doubling
            let numerator = ((x1.clone()
                * x1.clone()
                * FieldElement::new(BigUint::from(3u32), x1.field.clone()))
                + self.curve.a.clone());
            let denominator = y1.clone() * FieldElement::new(BigUint::from(2u32), y1.field.clone());
            numerator / denominator
        } else {
            // Point addition
            let numerator = y2.clone() - y1.clone();
            let denominator = x2.clone() - x1.clone();
            numerator / denominator
        };

        let x3 = (m.clone() * m.clone()) - x1.clone() - x2.clone();
        let y3 = (m.clone() * (x1.clone() - x3.clone())) - y1.clone();

        Point {
            x: Some(x3),
            y: Some(y3),
            curve: self.curve.clone(),
        }
    }
}

impl Divisor {
    fn new() -> Self {
        Divisor { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point, multiplicity: i32) {
        if let Some(index) = self.points.iter().position(|(p, _)| p == &point) {
            self.points[index].1 += multiplicity;
            if self.points[index].1 == 0 {
                self.points.remove(index);
            }
        } else if multiplicity != 0 {
            self.points.push((point, multiplicity));
        }
    }

    fn subtract(&self, other: &Divisor) -> Divisor {
        let mut result = self.clone();
        for (point, mult) in &other.points {
            result.add_point(point.clone(), -mult);
        }
        result
    }
}

#[derive(Clone, Debug)]
struct Polynomial {
    coefficients: Vec<FieldElement>,
}

#[derive(Clone, Debug)]
struct FunctionField {
    curve: EllipticCurve,
}

#[derive(Clone, Debug)]
struct FunctionFieldElement {
    numerator: Polynomial,
    denominator: Polynomial,
    field: FunctionField,
}

impl Polynomial {
    fn new(coefficients: Vec<FieldElement>) -> Self {
        Polynomial { coefficients }
    }

    fn evaluate(&self, x: &FieldElement) -> FieldElement {
        let mut result = self.coefficients[0].clone();
        let mut power = x.clone();
        for coeff in self.coefficients.iter().skip(1) {
            result = result + (power.clone() * coeff.clone());
            power = power.clone() * x.clone();
        }
        result
    }
}

impl FunctionField {
    fn new(curve: EllipticCurve) -> Self {
        FunctionField { curve }
    }
}

impl FunctionFieldElement {
    fn new(numerator: Polynomial, denominator: Polynomial, field: FunctionField) -> Self {
        FunctionFieldElement {
            numerator,
            denominator,
            field,
        }
    }
}

// langrange polynomial
fn interpolate(x_coords: &[FieldElement], y_coords: &[FieldElement]) -> Polynomial {
    assert_eq!(x_coords.len(), y_coords.len());
    let n = x_coords.len();
    let mut result = Polynomial::new(vec![
        FieldElement::new(
            BigUint::zero(),
            x_coords[0].field.clone()
        );
        n
    ]);

    for i in 0..n {
        let mut term = Polynomial::new(vec![y_coords[i].clone()]);
        for j in 0..n {
            if i != j {
                let numerator = Polynomial::new(vec![
                    FieldElement::new(BigUint::zero(), x_coords[0].field.clone()),
                    FieldElement::new(BigUint::one(), x_coords[0].field.clone()),
                ]);
                let denominator = x_coords[i].clone() - x_coords[j].clone();
                term = multiply_polynomials(&term, &numerator);
                term = scalar_multiply_polynomial(&term, &denominator.inv());
            }
        }
        result = add_polynomials(&result, &term);
    }

    result
}

fn add_polynomials(p1: &Polynomial, p2: &Polynomial) -> Polynomial {
    let max_len = std::cmp::max(p1.coefficients.len(), p2.coefficients.len());
    let mut result =
        vec![FieldElement::new(BigUint::zero(), p1.coefficients[0].field.clone()); max_len];

    for (i, coeff) in result.iter_mut().enumerate() {
        if i < p1.coefficients.len() {
            *coeff = coeff.clone() + p1.coefficients[i].clone();
        }
        if i < p2.coefficients.len() {
            *coeff = coeff.clone() + p2.coefficients[i].clone();
        }
    }

    Polynomial::new(result)
}

fn multiply_polynomials(p1: &Polynomial, p2: &Polynomial) -> Polynomial {
    let len = p1.coefficients.len() + p2.coefficients.len() - 1;
    let mut result =
        vec![FieldElement::new(BigUint::zero(), p1.coefficients[0].field.clone()); len];

    for (i, c1) in p1.coefficients.iter().enumerate() {
        for (j, c2) in p2.coefficients.iter().enumerate() {
            result[i + j] = result[i + j].clone() + (c1.clone() * c2.clone());
        }
    }

    Polynomial::new(result)
}

fn scalar_multiply_polynomial(p: &Polynomial, scalar: &FieldElement) -> Polynomial {
    Polynomial::new(
        p.coefficients
            .iter()
            .map(|c| c.clone() * scalar.clone())
            .collect(),
    )
}
fn divisor_of_function(f: &FunctionFieldElement) -> Divisor {
    let mut divisor = Divisor::new();
    // Add zeros of the numerator
    for x in 0..f.field.curve.a.field.prime.clone().to_u32().unwrap() {
        let x_fe = FieldElement::new(BigUint::from(x), f.field.curve.a.field.clone());
        if f.numerator.evaluate(&x_fe).value == BigUint::zero() {
            let y = f.field.curve.sqrt(
                &((x_fe.clone() * x_fe.clone() * x_fe.clone())
                    + (f.field.curve.a.clone() * x_fe.clone())
                    + f.field.curve.b.clone()),
            );
            if let Some(y) = y {
                divisor.add_point(
                    Point {
                        x: Some(x_fe.clone()),
                        y: Some(y),
                        curve: f.field.curve.clone(),
                    },
                    1,
                );
            }
        }
    }
    // Subtract poles of the denominator
    for x in 0..f.field.curve.a.field.prime.clone().to_u32().unwrap() {
        let x_fe = FieldElement::new(BigUint::from(x), f.field.curve.a.field.clone());
        if f.denominator.evaluate(&x_fe).value == BigUint::zero() {
            let y = f.field.curve.sqrt(
                &((x_fe.clone() * x_fe.clone() * x_fe.clone())
                    + (f.field.curve.a.clone() * x_fe.clone())
                    + f.field.curve.b.clone()),
            );
            if let Some(y) = y {
                divisor.add_point(
                    Point {
                        x: Some(x_fe.clone()),
                        y: Some(y),
                        curve: f.field.curve.clone(),
                    },
                    -1,
                );
            }
        }
    }
    divisor
}

fn support(divisor: &Divisor) -> Vec<Point> {
    divisor
        .points
        .iter()
        .map(|(point, _)| point.clone())
        .collect()
}
pub fn run() {
    let mut rng = rand::thread_rng();
    let q: u32 = rng.gen_range(2..50);
    let q = BigUint::from(q);

    let fq = FiniteField::new(q.clone());

    let a = fq.random_element();
    let b = fq.random_element();

    let e = EllipticCurve::new(a, b);

    let o = Point {
        x: None,
        y: None,
        curve: e.clone(),
    };

    let mut points = Vec::new();
    let mut xcoords = Vec::new();
    let mut ycoords = Vec::new();

    let mut d = Divisor::new();

    for _ in 0..11 {
        let pi = e.random_point();
        if let (Some(x), Some(y)) = (&pi.x, &pi.y) {
            xcoords.push(x.clone());
            ycoords.push(y.clone());
        }
        points.push(pi.clone());
        d.add_point(pi, 1);
        d.add_point(o.clone(), -1);
    }

    println!(
        "Elliptic curve operations completed. {} points generated.",
        points.len()
    );

    let interpolated = interpolate(&xcoords, &ycoords);
    let f = FunctionField::new(e.clone());
    let l = FunctionFieldElement::new(
        add_polynomials(
            &Polynomial::new(vec![
                FieldElement::new(BigUint::zero(), fq.clone()),
                FieldElement::new(BigUint::from(0u32), fq.clone()),
            ]),
            &interpolated,
        ),
        Polynomial::new(vec![FieldElement::new(BigUint::one(), fq.clone())]),
        f,
    );

    let div_l = divisor_of_function(&l);
    let support_l = support(&div_l);
    println!("Support of Divisor(l): {:?}", support_l);

    let dd = div_l.subtract(&d);
    let support_dd = support(&dd);
    println!("Support of Dd: {:?}", support_dd);
}
