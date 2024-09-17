use num_bigint::BigInt;
use num_traits::{Euclid, ToBytes};
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

        Point { x, y }
    }

    //     How the Double-and-Add Algorithm Works
    // The algorithm is analogous to the way binary numbers are used in multiplication. Here’s a step-by-step explanation:

    // Binary Representation of the Scalar: The integer
    // 𝑚
    // m is first converted into its binary representation. For example, if
    // 𝑚
    // =
    // 13
    // m=13, its binary representation is 1101.

    // Initialization:

    // Initialize R to the identity element (point at infinity) of the elliptic curve.
    // Set a variable temp to the point
    // 𝑃
    // P (the point you want to multiply).
    // Process Each Bit: Iterate through each bit of the binary representation of
    // 𝑚
    // m, starting from the most significant bit (MSB) to the least significant bit (LSB).

    // For each bit:

    // Double Operation: Regardless of the bit value, double the current result point R. This corresponds to shifting left in binary multiplication.
    // Add Operation: If the current bit is 1, add the point temp to R. This corresponds to adding the point in binary multiplication when the bit is 1.
    // Update the Point:

    // After processing all bits, R will contain the result of
    // 𝑚
    // 𝑃
    // mP.

    fn scalar(&self, m: i32, p: &Point) -> Point {
        let mut r = p.clone(); // Initialize r with the point p itself
        let mut first_bit = false;
        let binary_string = format!("{:b}", m);
        println!("Binary representation of {}: {}", m, binary_string);
        // Iterate through each byte in the integer
        for char in binary_string.chars() {
            // Iterate through bytes in reverse

            if !first_bit {
                first_bit = true;
                continue;
            }

            // Iterate through each bit in the byte
            // Double the point regardless of the bit
            r = self.double(&r);

            if char == '1' {
                r = self.add(&r, p); // Add point p to r if bit is 1
            }
        }

        r
    }
}

pub fn run() {
    let fq = FiniteField::new(BigInt::from(1021));
    let a = BigInt::from(-3);
    let b = BigInt::from(-3);
    let e = EllipticCurve::new(fq.clone(), a, b);

    let p = Point {
        x: BigInt::from(379),
        y: BigInt::from(1011),
    };
    let m = 655;
    let r = e.scalar(m, &p);
    println!("[655]P {:?}  = ({:?}, {:?})", p, r.x, r.y);
}
