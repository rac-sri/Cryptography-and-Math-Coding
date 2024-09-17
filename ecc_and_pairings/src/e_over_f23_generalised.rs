use num_bigint::BigInt;
use num_traits::Euclid;
use num_traits::{One, Zero};
use std::ops::Rem;

#[derive(Clone, Debug, PartialEq)]
struct FiniteField {
    p: BigInt,
}

impl FiniteField {
    fn new(p: BigInt) -> Self {
        FiniteField { p }
    }

    fn add(&self, a: &BigInt, b: &BigInt) -> BigInt {
        (a + b).rem_euclid(&self.p)
    }

    fn sub(&self, a: &BigInt, b: &BigInt) -> BigInt {
        (a - b + &self.p).rem_euclid(&self.p)
    }

    fn mul(&self, a: &BigInt, b: &BigInt) -> BigInt {
        (a * b).rem_euclid(&self.p)
    }

    fn inv(&self, a: &BigInt) -> Option<BigInt> {
        let (g, x, _) = extended_gcd(a, &self.p);
        if g.is_one() {
            Some(x.rem_euclid(&self.p))
        } else {
            None
        }
    }

    fn div(&self, a: &BigInt, b: &BigInt) -> Option<BigInt> {
        self.inv(b).map(|b_inv| self.mul(a, &b_inv))
    }

    fn neg(&self, a: &BigInt) -> BigInt {
        (&self.p - a).rem_euclid(&self.p)
    }
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    // Bézout's identity
    // a⋅x+b⋅y=gcd(a,b)
    if b.is_zero() {
        // gcd(a,0)=a
        // a⋅1+0⋅0=a
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (g, x, y) = extended_gcd(b, &a.rem(b));
        // euclidena algorithm
        (g, y.clone(), x - (a / b) * y)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct EllipticCurve {
    field: FiniteField,
    a: BigInt,
    b: BigInt,
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: BigInt,
    y: BigInt,
}

impl EllipticCurve {
    fn new(field: FiniteField, a: BigInt, b: BigInt) -> Self {
        EllipticCurve { field, a, b }
    }

    fn add(&self, p: &Point, q: &Point) -> Point {
        if p == q {
            return self.double(p);
        }

        // y = lambda*x + v

        // lambda = Yq - Yp / Xq-Xp
        let lambda = self
            .field
            .div(&self.field.sub(&q.y, &p.y), &self.field.sub(&q.x, &p.x))
            .unwrap();

        // v = Yp - lambda*Xp
        let nu = self.field.sub(&p.y, &self.field.mul(&lambda, &p.x));

        // (Xr, Yr ) = (lambda^2 - Xp - Xq, - (lambda * Xr +v))
        let x = self.field.sub(
            &self.field.sub(&self.field.mul(&lambda, &lambda), &p.x),
            &q.x,
        );
        let y = self
            .field
            .neg(&self.field.add(&self.field.mul(&lambda, &x), &nu));

        println!("lambda = {:?}, nu = {:?}", lambda, nu);

        Point { x, y }
    }

    fn double(&self, p: &Point) -> Point {
        let lambda = self
            .field
            .div(
                &self.field.add(
                    &self
                        .field
                        .mul(&BigInt::from(3), &self.field.mul(&p.x, &p.x)),
                    &self.a,
                ),
                &self.field.mul(&BigInt::from(2), &p.y),
            )
            .unwrap();

        let nu = self.field.sub(&p.y, &self.field.mul(&lambda, &p.x));

        let x = self.field.sub(
            &self.field.mul(&lambda, &lambda),
            &self.field.mul(&BigInt::from(2), &p.x),
        );

        let y = self
            .field
            .neg(&self.field.add(&self.field.mul(&lambda, &x), &nu));

        println!("lambda = {:?}, nu = {:?}", lambda, nu);

        Point { x, y }
    }
}

pub fn run() {
    let fq = FiniteField::new(BigInt::from(23));
    let a = BigInt::from(5);
    let b = BigInt::from(7);
    let e = EllipticCurve::new(fq.clone(), a, b);

    let p = Point {
        x: BigInt::from(2),
        y: BigInt::from(5),
    };
    let q = Point {
        x: BigInt::from(12),
        y: BigInt::from(1),
    };

    let r = e.add(&p, &q);
    println!("P + Q = ({:?}, {:?})", r.x, r.y);

    let s = e.double(&p);
    println!("2P = ({:?}, {:?})", s.x, s.y);
}
