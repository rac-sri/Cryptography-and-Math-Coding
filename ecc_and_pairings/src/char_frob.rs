use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Clone, Debug, PartialEq)]
struct FiniteField {
    value: BigUint,
    modulus: BigUint,
}

impl FiniteField {
    fn new(value: BigUint, modulus: BigUint) -> Self {
        FiniteField {
            value: value % &modulus,
            modulus,
        }
    }

    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus);
        FiniteField::new(&self.value + &other.value, self.modulus.clone())
    }

    fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus);
        let result = if self.value >= other.value {
            &self.value - &other.value
        } else {
            &self.modulus + &self.value - &other.value
        };
        FiniteField::new(result, self.modulus.clone())
    }

    fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus);
        FiniteField::new(&self.value * &other.value, self.modulus.clone())
    }

    fn pow(&self, exp: &BigUint) -> Self {
        let mut result = FiniteField::new(BigUint::one(), self.modulus.clone());
        let mut base = self.clone();
        let mut exp = exp.clone();

        while exp > BigUint::zero() {
            if &exp % 2u32 == BigUint::one() {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exp /= 2u32;
        }
        result
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ExtensionField2 {
    a: FiniteField,
    b: FiniteField,
}

impl ExtensionField2 {
    fn new(a: FiniteField, b: FiniteField) -> Self {
        assert_eq!(a.modulus, b.modulus);
        ExtensionField2 { a, b }
    }

    fn add(&self, other: &Self) -> Self {
        ExtensionField2 {
            a: self.a.add(&other.a),
            b: self.b.add(&other.b),
        }
    }

    fn mul(&self, other: &Self) -> Self {
        // (a + bu)(c + du) = (ac - bd) + (ad + bc)u
        let ac = self.a.mul(&other.a);
        let bd = self.b.mul(&other.b);
        let ad_bc = self.a.mul(&other.b).add(&self.b.mul(&other.a));
        ExtensionField2 {
            a: ac.sub(&bd), // ac - bd
            b: ad_bc,       // ad + bc
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ExtensionField3 {
    a: FiniteField,
    b: FiniteField,
    c: FiniteField,
}

impl ExtensionField3 {
    fn new(a: FiniteField, b: FiniteField, c: FiniteField) -> Self {
        assert_eq!(a.modulus, b.modulus);
        assert_eq!(a.modulus, c.modulus);
        ExtensionField3 { a, b, c }
    }

    fn add(&self, other: &Self) -> Self {
        ExtensionField3 {
            a: self.a.add(&other.a),
            b: self.b.add(&other.b),
            c: self.c.add(&other.c),
        }
    }

    fn mul(&self, other: &Self) -> Self {
        // (a + bv + cv^2)(d + ev + fv^2) =
        // (ad - 2bf - 2ce) + (ae + bd - 2cf)v + (af + be + cd)v^2
        let ad = self.a.mul(&other.a);
        let bf = self.b.mul(&other.c);
        let ce = self.c.mul(&other.b);
        let ae = self.a.mul(&other.b);
        let bd = self.b.mul(&other.a);
        let cf = self.c.mul(&other.c);
        let af = self.a.mul(&other.c);
        let be = self.b.mul(&other.b);
        let cd = self.c.mul(&other.a);

        ExtensionField3 {
            a: ad
                .sub(&bf.mul(&FiniteField::new(
                    BigUint::from(2u32),
                    self.a.modulus.clone(),
                )))
                .sub(&ce.mul(&FiniteField::new(
                    BigUint::from(2u32),
                    self.a.modulus.clone(),
                ))),
            b: ae.add(&bd).sub(&cf.mul(&FiniteField::new(
                BigUint::from(2u32),
                self.a.modulus.clone(),
            ))),
            c: af.add(&be).add(&cd),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct EllipticCurvePoint<T> {
    x: T,
    y: T,
    a: T,
    b: T,
}

impl<T: Clone> EllipticCurvePoint<T> {
    fn new(x: T, y: T, a: T, b: T) -> Self {
        EllipticCurvePoint { x, y, a, b }
    }
}

impl EllipticCurvePoint<FiniteField> {
    fn frobenius(&self) -> Self {
        let q = &self.x.modulus;
        EllipticCurvePoint::new(self.x.pow(q), self.y.pow(q), self.a.clone(), self.b.clone())
    }
}

impl EllipticCurvePoint<ExtensionField2> {
    fn frobenius(&self) -> Self {
        let q = &self.x.a.modulus;
        EllipticCurvePoint::new(
            ExtensionField2::new(
                self.x.a.pow(q),
                self.x
                    .b
                    .pow(q)
                    .mul(&FiniteField::new(BigUint::from(66u32), q.clone())),
            ),
            ExtensionField2::new(
                self.y.a.pow(q),
                self.y
                    .b
                    .pow(q)
                    .mul(&FiniteField::new(BigUint::from(66u32), q.clone())),
            ),
            self.a.clone(),
            self.b.clone(),
        )
    }
}

impl EllipticCurvePoint<ExtensionField3> {
    fn frobenius(&self) -> Self {
        let q = &self.x.a.modulus;
        let omega = FiniteField::new(BigUint::from(45u32), q.clone());
        EllipticCurvePoint::new(
            ExtensionField3::new(
                self.x.a.pow(q),
                self.x.b.pow(q).mul(&omega),
                self.x.c.pow(q).mul(&omega.pow(&BigUint::from(2u32))),
            ),
            ExtensionField3::new(
                self.y.a.pow(q),
                self.y.b.pow(q).mul(&omega),
                self.y.c.pow(q).mul(&omega.pow(&BigUint::from(2u32))),
            ),
            self.a.clone(),
            self.b.clone(),
        )
    }
}

fn trace_of_frobenius(a: &FiniteField, b: &FiniteField) -> BigUint {
    let q = &a.modulus;
    let t = q + BigUint::one() - BigUint::from(4u32);
    t
}
pub fn run() {
    let q = BigUint::from(67u32);
    let a = FiniteField::new(BigUint::from(4u32), q.clone());
    let b = FiniteField::new(BigUint::from(3u32), q.clone());

    let p = EllipticCurvePoint::new(
        FiniteField::new(BigUint::from(15u32), q.clone()),
        FiniteField::new(BigUint::from(50u32), q.clone()),
        a.clone(),
        b.clone(),
    );

    let pi_p = p.frobenius();
    println!("pi(P) == P: {}", pi_p == p);

    let t = trace_of_frobenius(&a, &b);
    println!("Trace of Frobenius: {}", t);

    // pi(pi(P)) - t*pi(P) + q*P
    // This operation is not fully implemented as it requires point addition and scalar multiplication

    // P2 in Fq2
    let u = ExtensionField2::new(
        FiniteField::new(BigUint::from(16u32), q.clone()),
        FiniteField::new(BigUint::from(2u32), q.clone()),
    );
    let p2 = EllipticCurvePoint::new(
        u.clone(),
        ExtensionField2::new(
            FiniteField::new(BigUint::from(39u32), q.clone()),
            FiniteField::new(BigUint::from(30u32), q.clone()),
        ),
        ExtensionField2::new(a.clone(), FiniteField::new(BigUint::zero(), q.clone())),
        ExtensionField2::new(b.clone(), FiniteField::new(BigUint::zero(), q.clone())),
    );

    let pi_pi_p2 = p2.frobenius().frobenius();
    println!("pi(pi(P2)) == P2: {}", pi_pi_p2 == p2);

    // P3 in Fq3
    let v = ExtensionField3::new(
        FiniteField::new(BigUint::from(8u32), q.clone()),
        FiniteField::new(BigUint::from(49u32), q.clone()),
        FiniteField::new(BigUint::from(19u32), q.clone()),
    );
    let p3 = EllipticCurvePoint::new(
        v.clone(),
        ExtensionField3::new(
            FiniteField::new(BigUint::from(21u32), q.clone()),
            FiniteField::new(BigUint::from(66u32), q.clone()),
            FiniteField::new(BigUint::from(20u32), q.clone()),
        ),
        ExtensionField3::new(
            a.clone(),
            FiniteField::new(BigUint::zero(), q.clone()),
            FiniteField::new(BigUint::zero(), q.clone()),
        ),
        ExtensionField3::new(
            b.clone(),
            FiniteField::new(BigUint::zero(), q.clone()),
            FiniteField::new(BigUint::zero(), q.clone()),
        ),
    );

    let pi_pi_p3 = p3.frobenius().frobenius();
    // pi(pi(P3)) - t*pi(P3) + q*P3
    // This operation is not fully implemented as it requires point addition and scalar multiplication in Fq3
}
