use std::ops::{Add, Mul, Neg};

// Field element in F_101
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FieldElement {
    value: i32,
}

impl FieldElement {
    fn new(value: i32) -> Self {
        FieldElement {
            value: value.rem_euclid(101),
        }
    }

    fn pow(&self, exp: u32) -> Self {
        let mut result = FieldElement::new(1);
        let mut base = *self;
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }
        result
    }

    fn sqrt(&self) -> Option<Self> {
        // Simplified square root for F_101 (a prime field where p ≡ 3 (mod 4))
        if self.value == 0 {
            return Some(FieldElement::new(0));
        }
        let result = self.pow((101 + 1) / 4);
        if result * result == *self {
            Some(result)
        } else {
            None
        }
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        FieldElement::new(self.value + other.value)
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        FieldElement::new(self.value * other.value)
    }
}

impl Neg for FieldElement {
    type Output = Self;
    fn neg(self) -> Self {
        FieldElement::new(-self.value)
    }
}

// Polynomial over F_101
#[derive(Clone, Debug)]
struct Polynomial {
    coeffs: Vec<FieldElement>,
}

impl Polynomial {
    fn new(coeffs: Vec<FieldElement>) -> Self {
        Polynomial { coeffs }
    }

    fn evaluate(&self, x: FieldElement) -> FieldElement {
        self.coeffs
            .iter()
            .rev()
            .fold(FieldElement::new(0), |acc, &coeff| acc * x + coeff)
    }

    fn scalar_mul(&self, scalar: FieldElement) -> Self {
        Polynomial::new(self.coeffs.iter().map(|&c| c * scalar).collect())
    }
}

impl Add for Polynomial {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let max_len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![FieldElement::new(0); max_len];
        for (i, &coeff) in self.coeffs.iter().enumerate() {
            result[i] = result[i] + coeff;
        }
        for (i, &coeff) in other.coeffs.iter().enumerate() {
            result[i] = result[i] + coeff;
        }
        Polynomial::new(result)
    }
}

impl Mul for Polynomial {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let n = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result = vec![FieldElement::new(0); n];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                result[i + j] = result[i + j] + a * b;
            }
        }
        Polynomial::new(result)
    }
}

impl Neg for Polynomial {
    type Output = Self;
    fn neg(self) -> Self {
        Polynomial::new(self.coeffs.into_iter().map(|c| -c).collect())
    }
}

// Elliptic curve y^2 = x^3 + ax + b over F_101
struct EllipticCurve {
    a: FieldElement,
    b: FieldElement,
}

impl EllipticCurve {
    fn new(a: FieldElement, b: FieldElement) -> Self {
        EllipticCurve { a, b }
    }

    fn division_polynomial(&self, n: u32) -> Polynomial {
        let x = Polynomial::new(vec![FieldElement::new(0), FieldElement::new(1)]);
        let y2 = Polynomial::new(vec![
            self.b,
            self.a,
            FieldElement::new(0),
            FieldElement::new(1),
        ]);

        match n {
            0 => Polynomial::new(vec![FieldElement::new(0)]),
            1 => Polynomial::new(vec![FieldElement::new(1)]),
            2 => Polynomial::new(vec![
                FieldElement::new(2),
                FieldElement::new(0),
                FieldElement::new(0),
            ]),
            3 => {
                let mut psi3 = Polynomial::new(vec![
                    self.a * FieldElement::new(3),
                    FieldElement::new(0),
                    FieldElement::new(3),
                ]);
                psi3 = psi3.scalar_mul(FieldElement::new(3));
                psi3
            }
            4 => {
                let mut psi4 = Polynomial::new(vec![
                    -self.b * FieldElement::new(4),
                    -self.a * FieldElement::new(2),
                    FieldElement::new(0),
                    FieldElement::new(4),
                ]);
                psi4 = psi4 * y2;
                psi4.scalar_mul(FieldElement::new(4))
            }
            _ => {
                let m = n / 2;
                if n % 2 == 1 {
                    let psi_m_plus_2 = self.division_polynomial(m + 2);
                    let psi_m_minus_1 = self.division_polynomial(m - 1);
                    let psi_m = self.division_polynomial(m);
                    let psi_m_plus_1 = self.division_polynomial(m + 1);

                    (psi_m_plus_2 * psi_m.clone() * psi_m.clone() * psi_m)
                        + (-psi_m_plus_1.clone()
                            * psi_m_plus_1.clone()
                            * psi_m_plus_1
                            * psi_m_minus_1)
                } else {
                    let psi_m_plus_1 = self.division_polynomial(m + 1);
                    let psi_m_minus_1 = self.division_polynomial(m - 1);
                    let psi_m = self.division_polynomial(m);

                    let half = FieldElement::new(51); // 51 * 2 ≡ 1 (mod 101)
                    (psi_m.clone().scalar_mul(half))
                        * ((psi_m_plus_1 * psi_m_minus_1) + (-y2 * psi_m.clone() * psi_m))
                }
            }
        }
    }

    fn find_roots(&self, poly: &Polynomial) -> Vec<FieldElement> {
        (0..101)
            .filter_map(|i| {
                let x = FieldElement::new(i);
                if poly.evaluate(x) == FieldElement::new(0) {
                    Some(x)
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_points(&self, x: FieldElement) -> Vec<(FieldElement, FieldElement)> {
        let y2 = x.pow(3) + self.a * x + self.b;
        match y2.sqrt() {
            Some(y) => vec![(x, y), (x, -y)],
            None => vec![],
        }
    }
}

pub fn run() {
    let e = EllipticCurve::new(FieldElement::new(1), FieldElement::new(1));

    let psi2 = e.division_polynomial(2);
    let psi3 = e.division_polynomial(3);
    let psi5 = e.division_polynomial(5);
    let psi7 = e.division_polynomial(7);
    let psi11 = e.division_polynomial(11);

    println!("Roots of psi2: {:?}", e.find_roots(&psi2));
    println!("Roots of psi3: {:?}", e.find_roots(&psi3));

    let roots3 = e.find_roots(&psi3);
    println!("Points for roots of psi3:");
    for root in roots3 {
        println!("{:?}", e.find_points(root));
    }

    println!("Roots of psi5: {:?}", e.find_roots(&psi5));

    let roots5 = e.find_roots(&psi5);
    println!("Points for roots of psi5:");
    for root in roots5 {
        println!("{:?}", e.find_points(root));
    }

    println!("Roots of psi7: {:?}", e.find_roots(&psi7));

    let roots7 = e.find_roots(&psi7);
    println!("Points for roots of psi7:");
    for root in roots7 {
        println!("{:?}", e.find_points(root));
    }

    println!("Roots of psi11: {:?}", e.find_roots(&psi11));
}
