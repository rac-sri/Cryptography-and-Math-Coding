use rand::Rng;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fq(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fq2 {
    real: Fq,
    imag: Fq,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: Fq2,
    y: Fq2,
    z: Fq2, // For projective coordinates
}

#[derive(Debug)]
struct EllipticCurve {
    a: Fq,
    b: Fq,
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

    fn pow(&self, exp: u64, q: u64) -> Fq {
        let mut result = Fq(1);
        let mut base = *self;
        let mut exp = exp;

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
        // Extended Euclidean algorithm
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

impl Fq2 {
    fn new(real: Fq, imag: Fq) -> Self {
        Fq2 { real, imag }
    }

    fn add(&self, other: &Fq2, q: u64) -> Fq2 {
        Fq2 {
            real: self.real.add(&other.real, q),
            imag: self.imag.add(&other.imag, q),
        }
    }

    fn sub(&self, other: &Fq2, q: u64) -> Fq2 {
        Fq2 {
            real: self.real.sub(&other.real, q),
            imag: self.imag.sub(&other.imag, q),
        }
    }

    fn mul(&self, other: &Fq2, q: u64) -> Fq2 {
        let ac = self.real.mul(&other.real, q);
        let bd = self.imag.mul(&other.imag, q);
        let ad = self.real.mul(&other.imag, q);
        let bc = self.imag.mul(&other.real, q);

        Fq2 {
            real: ac.sub(&bd, q),
            imag: ad.add(&bc, q),
        }
    }

    fn square(&self, q: u64) -> Fq2 {
        let a2 = self.real.mul(&self.real, q);
        let b2 = self.imag.mul(&self.imag, q);
        let ab2 = self.real.mul(&self.imag, q).mul(&Fq(2), q);

        Fq2 {
            real: a2.sub(&b2, q),
            imag: ab2,
        }
    }

    fn inv(&self, q: u64) -> Option<Fq2> {
        // (a + bi)^(-1) = (a - bi)/(a^2 + b^2)
        let norm = self
            .real
            .mul(&self.real, q)
            .add(&self.imag.mul(&self.imag, q), q);
        let norm_inv = norm.inv(q)?;

        Some(Fq2 {
            real: self.real.mul(&norm_inv, q),
            imag: Fq((q - self.imag.0) % q).mul(&norm_inv, q),
        })
    }
}

impl Point {
    fn new(x: Fq2, y: Fq2) -> Self {
        Point {
            x,
            y,
            z: Fq2::new(Fq(1), Fq(0)), // Affine coordinates initially
        }
    }

    fn identity() -> Self {
        Point {
            x: Fq2::new(Fq(0), Fq(0)),
            y: Fq2::new(Fq(1), Fq(0)),
            z: Fq2::new(Fq(0), Fq(0)),
        }
    }
}

impl EllipticCurve {
    fn new(a: Fq, b: Fq, q: u64) -> Self {
        EllipticCurve { a, b, q }
    }

    fn is_supersingular(&self) -> bool {
        // For prime q ≡ 3 mod 4, y^2 = x^3 + ax + b is supersingular
        // if and only if a = 0 and b ≠ 0
        self.a.0 == 0 && self.b.0 != 0 && self.q % 4 == 3
    }

    fn is_on_curve(&self, point: &Point) -> bool {
        if point.z.real.0 == 0 && point.z.imag.0 == 0 {
            return true; // Point at infinity is always on curve
        }

        let x3 = point.x.mul(&point.x, self.q).mul(&point.x, self.q);
        let ax = Fq2::new(self.a, Fq(0)).mul(&point.x, self.q);
        let rhs = x3.add(&ax, self.q).add(&Fq2::new(self.b, Fq(0)), self.q);
        let y2 = point.y.square(self.q);

        y2 == rhs
    }

    fn add(&self, p1: &Point, p2: &Point) -> Point {
        if p1.z.real.0 == 0 && p1.z.imag.0 == 0 {
            return p2.clone();
        }
        if p2.z.real.0 == 0 && p2.z.imag.0 == 0 {
            return p1.clone();
        }

        let p1x = p1.x;
        let p1y = p1.y;
        let p2x = p2.x;
        let p2y = p2.y;

        if p1x == p2x {
            if p1y == p2y {
                return self.double(p1);
            } else {
                return Point::identity();
            }
        }

        let slope = p2y
            .sub(&p1y, self.q)
            .mul(&p2x.sub(&p1x, self.q).inv(self.q).unwrap(), self.q);

        let x3 = slope.square(self.q).sub(&p1x, self.q).sub(&p2x, self.q);

        let y3 = slope.mul(&p1x.sub(&x3, self.q), self.q).sub(&p1y, self.q);

        Point::new(x3, y3)
    }

    fn double(&self, p: &Point) -> Point {
        if p.z.real.0 == 0 && p.z.imag.0 == 0 {
            return Point::identity();
        }

        let x = p.x;
        let y = p.y;

        if y.real.0 == 0 && y.imag.0 == 0 {
            return Point::identity();
        }

        let two_y = y.mul(&Fq2::new(Fq(2), Fq(0)), self.q);
        let two_y_inv = two_y.inv(self.q).unwrap();

        let x_squared = x.square(self.q);
        let three_x_squared = x_squared.mul(&Fq2::new(Fq(3), Fq(0)), self.q);

        let slope = three_x_squared
            .add(&Fq2::new(self.a, Fq(0)), self.q)
            .mul(&two_y_inv, self.q);

        let x3 = slope
            .square(self.q)
            .sub(&x.mul(&Fq2::new(Fq(2), Fq(0)), self.q), self.q);

        let y3 = slope.mul(&x.sub(&x3, self.q), self.q).sub(&y, self.q);

        Point::new(x3, y3)
    }

    fn scalar_mul(&self, k: u64, p: &Point) -> Point {
        let mut result = Point::identity();
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

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        points.push(Point::identity());

        for x_real in 0..self.q {
            for x_imag in 0..self.q {
                let x = Fq2::new(Fq(x_real), Fq(x_imag));
                let x3 = x.mul(&x, self.q).mul(&x, self.q);
                let ax = Fq2::new(self.a, Fq(0)).mul(&x, self.q);
                let rhs = x3.add(&ax, self.q).add(&Fq2::new(self.b, Fq(0)), self.q);

                for y_real in 0..self.q {
                    for y_imag in 0..self.q {
                        let y = Fq2::new(Fq(y_real), Fq(y_imag));
                        if y.square(self.q) == rhs {
                            points.push(Point::new(x, y));
                        }
                    }
                }
            }
        }

        points
    }
}

fn find_torsion_points(curve: &EllipticCurve, points: &Vec<Point>, r: u64) -> HashSet<Point> {
    let mut tors_pts = HashSet::new();
    for point in points {
        if curve.scalar_mul(r, point) == Point::identity() {
            tors_pts.insert(point.clone());
        }
    }

    tors_pts
}

fn flower_generator(tors_pts: HashSet<Point>) -> Vec<HashSet<Point>> {
    let mut petals = Vec::new();
    let mut remaining_points = tors_pts.clone();
    let petals_count = (tors_pts.len() as f64).sqrt() as usize + 1;
    let points_per_petal = petals_count - 1;

    let mut rng = rand::thread_rng();

    for _ in 0..petals_count {
        if remaining_points.is_empty() {
            break;
        }

        let random_point = remaining_points
            .iter()
            .filter(|p| p.z.real.0 != 0 || p.z.imag.0 != 0)
            .nth(rng.gen_range(0..remaining_points.len()))
            .unwrap()
            .clone();

        let mut petal = HashSet::new();

        for j in 1..=points_per_petal {
            petal.insert(random_point.clone());
        }

        for point in &petal {
            remaining_points.remove(point);
        }

        petals.push(petal);
    }
    petals
}
pub fn run() {
    let q: u64 = 59;
    let a = Fq::new(0, q);
    let b = Fq::new(1, q);

    let curve = EllipticCurve::new(a, b, q);

    println!("Is supersingular: {}", curve.is_supersingular());

    let r: u64 = 5;

    let points = curve.points();
    println!("Number of points: {}", points.len());

    let tors_pts = find_torsion_points(&curve, &points, r);
    println!("Number of torsion points: {}", tors_pts.len());

    let flower = flower_generator(tors_pts);
    println!("Number of flower petals: {}", flower.len());

    let zi3 = Fq2::new(Fq::new(29, q), Fq::new(24, q));
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let random_point = &points[rng.gen_range(0..points.len())];
        let multiplied_x = random_point.x.mul(&zi3, q);
        let new_point = Point::new(multiplied_x, random_point.y);

        println!("Original point: {:?}", random_point);
        println!("Multiplied point: {:?}", new_point);
    }
}
