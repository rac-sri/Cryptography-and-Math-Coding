use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fq(u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Fq6 {
    coeffs: Vec<Fq>, // Length 6 vector for x^5 + ... + x + constant
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: Fq6,
    y: Fq6,
    z: Fq6, // For projective coordinates
}

#[derive(Debug)]
struct EllipticCurve {
    a: Fq6,
    b: Fq6,
    q: u64,
}

impl Fq {
    fn new(value: u64, q: u64) -> Self {
        Fq(value % q)
    }

    fn add(&self, other: &Fq, q: u64) -> Fq {
        Fq((self.0 + other.0) % q)
    }

    fn sub(&self, other: &Fq, q: u64) -> Fq {
        Fq((self.0 + q - other.0) % q)
    }

    fn mul(&self, other: &Fq, q: u64) -> Fq {
        Fq((self.0 * other.0) % q)
    }

    fn pow(&self, mut exp: u64, q: u64) -> Fq {
        let mut base = *self;
        let mut result = Fq(1);

        while exp > 0 {
            if exp & 1 == 1 {
                result = result.mul(&base, q);
            }
            base = base.mul(&base, q);
            exp >>= 1;
        }
        result
    }

    fn inv(&self, q: u64) -> Option<Fq> {
        let mut t = 0i64;
        let mut newt = 1i64;
        let mut r = q as i64;
        let mut newr = self.0 as i64;

        while newr != 0 {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }

        if r > 1 {
            return None;
        }
        if t < 0 {
            t += q as i64;
        }
        Some(Fq(t as u64))
    }
}

impl Fq6 {
    fn zero(q: u64) -> Self {
        Fq6 {
            coeffs: vec![Fq(0); 6],
        }
    }

    fn one(q: u64) -> Self {
        let mut coeffs = vec![Fq(0); 6];
        coeffs[0] = Fq(1);
        Fq6 { coeffs }
    }

    fn new(coeffs: Vec<Fq>) -> Self {
        assert_eq!(coeffs.len(), 6);
        Fq6 { coeffs }
    }

    fn add(&self, other: &Fq6, q: u64) -> Fq6 {
        let mut result = vec![Fq(0); 6];
        for i in 0..6 {
            result[i] = self.coeffs[i].add(&other.coeffs[i], q);
        }
        Fq6::new(result)
    }

    fn sub(&self, other: &Fq6, q: u64) -> Fq6 {
        let mut result = vec![Fq(0); 6];
        for i in 0..6 {
            result[i] = self.coeffs[i].sub(&other.coeffs[i], q);
        }
        Fq6::new(result)
    }

    fn mul(&self, other: &Fq6, q: u64) -> Fq6 {
        let mut result = vec![Fq(0); 11];
        for i in 0..6 {
            for j in 0..6 {
                let prod = self.coeffs[i].mul(&other.coeffs[j], q);
                result[i + j] = result[i + j].add(&prod, q);
            }
        }

        // Reduce modulo x^6 + 2
        let mut final_result = vec![Fq(0); 6];
        for i in 0..6 {
            final_result[i] = result[i];
        }
        for i in 6..11 {
            let coeff = result[i];
            let power = i - 6;
            final_result[power] = final_result[power].sub(&coeff.mul(&Fq(2), q), q);
        }

        Fq6::new(final_result)
    }

    fn pow(&self, mut exp: u64, q: u64) -> Fq6 {
        let mut base = self.clone();
        let mut result = Fq6::one(q);

        while exp > 0 {
            if exp & 1 == 1 {
                result = result.mul(&base, q);
            }
            base = base.mul(&base, q);
            exp >>= 1;
        }
        result
    }
}

impl Point {
    fn new(x: Fq6, y: Fq6) -> Self {
        Point {
            x: x.clone(),
            y,
            z: Fq6::one(x.coeffs[0].0), // Assuming q is the same for all Fq elements
        }
    }

    fn identity(q: u64) -> Self {
        Point {
            x: Fq6::zero(q),
            y: Fq6::one(q),
            z: Fq6::zero(q),
        }
    }
}

impl EllipticCurve {
    fn new(a: Fq6, b: Fq6, q: u64) -> Self {
        EllipticCurve { a, b, q }
    }

    fn add(&self, p1: &Point, p2: &Point) -> Point {
        if p1.z == Fq6::zero(self.q) {
            return p2.clone();
        }
        if p2.z == Fq6::zero(self.q) {
            return p1.clone();
        }

        if p1.x == p2.x {
            if p1.y == p2.y {
                return self.double(p1);
            } else {
                return Point::identity(self.q);
            }
        }

        let slope =
            p2.y.sub(&p1.y, self.q)
                .mul(&p2.x.sub(&p1.x, self.q).pow(self.q - 2, self.q), self.q);

        let x3 = slope
            .mul(&slope, self.q)
            .sub(&p1.x, self.q)
            .sub(&p2.x, self.q);

        let y3 = slope.mul(&p1.x.sub(&x3, self.q), self.q).sub(&p1.y, self.q);

        Point::new(x3, y3)
    }

    fn double(&self, p: &Point) -> Point {
        if p.z == Fq6::zero(self.q) {
            return Point::identity(self.q);
        }

        if p.y == Fq6::zero(self.q) {
            return Point::identity(self.q);
        }

        let slope =
            p.x.mul(&p.x, self.q)
                .mul(
                    &Fq6::new(vec![Fq(3), Fq(0), Fq(0), Fq(0), Fq(0), Fq(0)]),
                    self.q,
                )
                .add(&self.a, self.q)
                .mul(
                    &p.y.mul(
                        &Fq6::new(vec![Fq(2), Fq(0), Fq(0), Fq(0), Fq(0), Fq(0)]),
                        self.q,
                    )
                    .pow(self.q - 2, self.q),
                    self.q,
                );

        let x3 = slope.mul(&slope, self.q).sub(
            &p.x.mul(
                &Fq6::new(vec![Fq(2), Fq(0), Fq(0), Fq(0), Fq(0), Fq(0)]),
                self.q,
            ),
            self.q,
        );

        let y3 = slope.mul(&p.x.sub(&x3, self.q), self.q).sub(&p.y, self.q);

        Point::new(x3, y3)
    }

    fn scalar_mul(&self, k: u64, p: &Point) -> Point {
        let mut result = Point::identity(self.q);
        let mut temp = p.clone();
        let mut k = k;

        while k > 0 {
            if k & 1 == 1 {
                result = self.add(&result, &temp);
            }
            temp = self.double(&temp);
            k >>= 1;
        }

        result
    }

    // If E is an elliptic curve y^2 = x^3 + ax + b, then the twisted curve E' has equation y^2 = x^3 + a(u^4)x + b(u^6)
    fn twist(&self, p: &Point, u: &Fq6) -> Point {
        let u2 = u.mul(u, self.q); // u^2
        let u3 = u2.mul(u, self.q); // u^3

        Point::new(
            p.x.mul(&u2, self.q), // x' = x * u^2
            p.y.mul(&u3, self.q),
        ) // y' = y * u^3
    }

    fn untwist(&self, p: &Point, u: &Fq6) -> Point {
        // We use u^(-2) and u^(-3) in untwisting because these are the inverses of u^2 and u^3
        let u2 = u.mul(u, self.q); // u^2
        let u3 = u2.mul(u, self.q); // u^3
        let u2_inv = u2.pow(self.q - 2, self.q); // (u^2)^(-1)
        let u3_inv = u3.pow(self.q - 2, self.q); // (u^3)^(-1)
        Point::new(
            p.x.mul(&u2_inv, self.q), // x = x' * (u^2)^(-1)
            p.y.mul(&u3_inv, self.q),
        ) // y = y' * (u^3)^(-1)
    }
}
fn flower_generator(tors_pts: HashSet<Point>, curve: &EllipticCurve) -> Vec<HashSet<Point>> {
    let mut petals = Vec::new();
    let mut remaining_points = tors_pts.clone();
    let petals_count = (tors_pts.len() as f64).sqrt() as usize + 1;
    let points_per_petal = petals_count - 1;

    let mut rng = rand::thread_rng();

    while petals.len() < petals_count && !remaining_points.is_empty() {
        // Find a random non-zero point
        let random_point = remaining_points
            .iter()
            .filter(|p| p.z != Fq6::zero(curve.q))
            .nth(rng.gen_range(0..remaining_points.len()))
            .unwrap()
            .clone();

        let mut petal = HashSet::new();

        // Generate points for this petal
        for j in 1..=points_per_petal {
            let point = curve.scalar_mul(j as u64, &random_point);
            petal.insert(point.clone());
            remaining_points.remove(&point);
        }

        petals.push(petal);
    }

    petals
}
pub fn run() {
    let q: u64 = 103;
    let r: u64 = 7;

    // Create base field Fq
    let a = Fq6::zero(q);
    let b = Fq6::new(vec![Fq(72), Fq(0), Fq(0), Fq(0), Fq(0), Fq(0)]);

    let curve = EllipticCurve::new(a.clone(), b.clone(), q);

    // Create extension field Fq6 and u
    let u = Fq6::new(vec![Fq(0), Fq(1), Fq(0), Fq(0), Fq(0), Fq(0)]); // Representing x

    // Create twisted curve
    let b_twist = b.mul(&u.pow(6, q), q);
    let curve_twist = EllipticCurve::new(a.clone(), b_twist, q);

    // Generate torsion points
    let mut tors_pts = HashSet::new();
    let mut tors_pts_twist = HashSet::new();

    let h = 1; // This should be properly calculated
    let mut rng = rand::thread_rng();
    while tors_pts.len() < (r * r) as usize {
        // Generate a random point by randomizing x and y
        let x_rand = Fq6::new(
            (0..6)
                .map(|_| Fq(rng.gen_range(0..q))) // Random Fq6 element for x
                .collect(),
        );
        let y_rand = Fq6::new(
            (0..6)
                .map(|_| Fq(rng.gen_range(0..q))) // Random Fq6 element for y
                .collect(),
        );

        let p = Point::new(x_rand, y_rand);

        // Multiply by cofactor (h)
        let hp = curve.scalar_mul(h, &p);
        if !tors_pts.contains(&hp) {
            for i in 1..=6 {
                tors_pts.insert(curve.scalar_mul(i as u64, &hp));
            }
        }
    }

    // Similar for twist points
    while tors_pts_twist.len() < (r * r) as usize {
        let x_rand = Fq6::new(
            (0..6)
                .map(|_| Fq(rng.gen_range(0..q))) // Random Fq6 element for x
                .collect(),
        );
        let y_rand = Fq6::new(
            (0..6)
                .map(|_| Fq(rng.gen_range(0..q))) // Random Fq6 element for y
                .collect(),
        );

        let p = Point::new(x_rand, y_rand);

        // Multiply by cofactor (h)
        let hp = curve_twist.scalar_mul(h, &p);
        if !tors_pts_twist.contains(&hp) {
            for i in 1..=6 {
                tors_pts_twist.insert(curve_twist.scalar_mul(i as u64, &hp));
            }
        }
    }

    // Generate flowers
    let flower = flower_generator(tors_pts.clone(), &curve);
    let flower_twist = flower_generator(tors_pts_twist.clone(), &curve_twist);

    println!("Flower petals: {}", flower.len());
    println!("Twisted flower petals: {}", flower_twist.len());

    // Test twisting and untwisting
    let mut rng = rand::thread_rng();
    let random_point = tors_pts
        .iter()
        .nth(rng.gen_range(0..tors_pts.len()))
        .unwrap();

    println!("Original point: {:?}", random_point);
    let twisted = curve.twist(random_point, &u);
    println!("Twisted point: {:?}", twisted);
    let untwisted = curve_twist.untwist(&twisted, &u);
    println!("Untwisted point: {:?}", untwisted);
    // TODO: fix
    // assert_eq!(*random_point, untwisted);
}
