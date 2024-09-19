use num_bigint::BigInt;
use num_traits::{One, Zero};

fn all_roots(a: &BigInt, n: &BigInt, p: &BigInt) -> Vec<BigInt> {
    let mut roots = Vec::new();
    let one = BigInt::one();

    // In a finite field GF(p) where p is prime, the multiplicative group has order p-1.
    // This means there are p-1 non-zero elements in the field.
    // If we're looking for nth roots, we're essentially dividing this multiplicative group into n subgroups.
    // So, q = (p-1) / n represents the size of each of these subgroups.
    let q = (p - &one) / n;

    let mut i = BigInt::zero();
    while &i < n {
        // x = a^((q*i + 1) mod p)
        // The formula a^((q*i + 1) mod p) is essentially trying different values of k to find all possible nth roots.
        let x = mod_pow(a, &(&q * &i + &one), p);
        // check x^n ≡ a (mod p)
        if mod_pow(&x, n, p) == *a {
            roots.push(x);
        }
        i += 1;
    }
    roots
}

fn mod_pow(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    let mut result = BigInt::one();
    let mut base = base.clone();
    let mut exp = exp.clone();

    while exp > BigInt::zero() {
        if &exp % 2 == BigInt::one() {
            result = (result * &base) % modulus;
        }

        base = (&base * &base) % modulus;
        exp /= 2;
    }

    result
}

fn mod_inv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    // a⋅x≡1 (mod m)
    // gcd(a, m) = 1 i.e. coprime

    // Extended euclidean algorithm: a⋅x+m⋅y=gcd(a,m)=1

    // t and newt tracks coefficient in extended field
    let mut t = BigInt::zero();
    let mut newt = BigInt::one();
    // variables to track remainders in the euclidean division process
    let mut r = m.clone();
    let mut newr = a.clone();

    // The loop continues until the remainder newr becomes zero, which means r now holds the gcd(a, m)
    while newr != BigInt::zero() {
        let quotient = &r / &newr;
        // t and newt are updated to track the coefficients for a and m (similar to x and y in the Extended Euclidean Algorithm)
        t = t - &quotient * &newt;
        // The swap operations alternate the values of t/newt and r/newr after each iteration.
        std::mem::swap(&mut t, &mut newt);
        r = r - &quotient * &newr;
        std::mem::swap(&mut r, &mut newr);
    }

    if r > BigInt::one() {
        None
    } else {
        if t < BigInt::zero() {
            t += m;
        }
        Some(t)
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: BigInt,
    y: BigInt,
    z: BigInt,
}

impl Point {
    fn new(x: BigInt, y: BigInt) -> Self {
        Point {
            x,
            y,
            z: BigInt::one(),
        }
    }
}

struct EllipticCurve {
    a: BigInt,
    b: BigInt,
    p: BigInt,
}

impl EllipticCurve {
    fn new(a: BigInt, b: BigInt, p: BigInt) -> Self {
        EllipticCurve { a, b, p }
    }

    fn add(&self, p1: &Point, p2: &Point) -> Point {
        if p1.z == BigInt::zero() {
            return p2.clone();
        }
        if p2.z == BigInt::zero() {
            return p1.clone();
        }

        let x1 = &p1.x;
        let y1 = &p1.y;
        let x2 = &p2.x;
        let y2 = &p2.y;

        if x1 == x2 && y1 != y2 {
            return Point {
                x: BigInt::zero(),
                y: BigInt::one(),
                z: BigInt::zero(),
            };
        }

        let m = if x1 == x2 && y1 == y2 {
            (BigInt::from(3) * x1 * x1 + &self.a)
                * mod_inv(&(BigInt::from(2) * y1), &self.p).unwrap()
        } else {
            (y2 - y1) * mod_inv(&(x2 - x1), &self.p).unwrap()
        };

        let x3 = (&m * &m - x1 - x2) % &self.p;
        let y3 = (&m * (x1 - &x3) - y1) % &self.p;

        Point::new((x3 + &self.p) % &self.p, (y3 + &self.p) % &self.p)
    }

    // sclare multiplication
    pub fn multiply(&self, k: &BigInt, p: &Point) -> Point {
        let mut result = Point {
            x: BigInt::zero(),
            y: BigInt::one(),
            z: BigInt::zero(),
        };
        let mut temp = p.clone();
        let mut k = k.clone();

        // double and add algorithm
        while k > BigInt::zero() {
            if &k % 2 == BigInt::one() {
                result = self.add(&result, &temp);
            }
            temp = self.add(&temp, &temp);
            k /= 2;
        }
        result
    }
}

pub fn run() {
    let q = BigInt::from(19);
    let a = BigInt::from(0);
    let b = BigInt::from(5);
    let e = EllipticCurve::new(a.clone(), b.clone(), q.clone());
    let p = Point::new(BigInt::from(-1), BigInt::from(2));

    let roots = all_roots(&BigInt::one(), &BigInt::from(3), &q);
    let zi3 = &roots[1];

    let result1 = e.multiply(zi3, &p);
    println!("Result 1: ({}, {})", result1.x, result1.y);

    // Second part
    let q = BigInt::from(23);
    let a = BigInt::from(0);
    let b = BigInt::from(5);
    let e = EllipticCurve::new(a.clone(), b.clone(), q.clone());
    let p = Point::new(BigInt::from(-1), BigInt::from(2));

    // Note: For Fq2, we would need to implement a more complex field extension structure.
    // This implementation only works with the base field GF(q).

    let roots = all_roots(&BigInt::one(), &BigInt::from(3), &q);
    println!("Roots in GF({}): {:?}", q, roots);
}
