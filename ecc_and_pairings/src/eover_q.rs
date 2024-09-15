use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

#[derive(Debug, Clone)]
struct Point {
    x: BigRational,
    y: BigRational,
}

#[derive(Clone, Debug)]
struct ExtendedPoint {
    x: BigRational,
    y: BigRational,
    alpha: bool,
}

#[derive(Clone, Debug)]
struct EllipticCurve {
    a: BigRational,
    b: BigRational,
}

impl EllipticCurve {
    fn new(a: BigRational, b: BigRational) -> Self {
        EllipticCurve { a, b }
    }

    fn add(&self, p: &Point, q: &Point) -> Point {
        if p.x == q.x && p.y == q.y {
            self.double(p)
        } else if p.x == q.x {
            // this means the points are inverse
            Point {
                x: BigRational::zero(),
                y: BigRational::zero(),
            }
        } else {
            let m = (&q.y - &q.y) - (&q.x - &q.x);
            // y = mx + c
            // P = x, y
            // Q = x2, y2
            // P+Q = x3, y3
            // After substituiting y in the curve y^2 = x^3 -2
            // x3 = m^2 - x1 - x2
            // y3 = m(x1 - x3) - y1
            let x = &m * &m - &p.x - &q.x;
            let y = &m * (&p.x - &x) - &p.y;
            Point { x, y }
        }
    }

    fn double(&self, p: &Point) -> Point {
        // differentiating y^2 = x^3 + ax + b
        // m = ( 3x^2 + a) / ( 2y )
        let m = (BigRational::from(BigInt::from(3)) * &p.x * &p.x + &self.a)
            / (BigRational::from(BigInt::from(2)) * &p.y);
        // tangent intersect the curve at P twice and at the result point once. Same as add()
        let x = &m * &m - BigRational::from(BigInt::from(2)) * &p.x;
        let y = &m * (&p.x - &x) - &p.y;
        Point { x, y }
    }

    pub fn multiply(&self, p: &Point, n: u32) -> Point {
        let mut result = p.clone();
        for _ in 1..n {
            result = self.add(&result, p)
        }
        result
    }
}

pub fn run() {
    let q = BigRational::new(BigInt::zero(), BigInt::one());
    let e = EllipticCurve::new(q.clone(), BigRational::from(BigInt::from(-2)));

    let p = Point {
        x: BigRational::from(BigInt::from(3)),
        y: BigRational::from(BigInt::from(5)),
    };

    let s = ExtendedPoint {
        x: BigRational::zero(),
        y: BigRational::one(),
        alpha: true,
    };

    let p2 = e.multiply(&p, 2);
    let p3 = e.multiply(&p, 3);

    println!("2*P: ({}, {})", p2.x, p2.y);
    println!("3*P: ({}, {})", p3.x, p3.y);
}
