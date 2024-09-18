use rand::Rng;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EllipticCurve {
    a: i32,
    b: i32,
}

// The discriminant of an equation is a quantity that provides information about the nature of the roots or singularities of the equation.
// For an elliptic curve of the form \( y^2 = x^3 + ax + b \), the discriminant helps us determine whether the curve is **non-singular**, meaning it does not have cusps or self-intersections.
// For an elliptic curve given by \( y^2 = x^3 + ax + b \), the **discriminant** \( \Delta \) is:
// Delta = 4a^3 + 27b^2
// If Delta!= 0, the curve is non-singular, meaning it behaves nicely and has no cusps or self-intersections. This ensures the curve can be used for elliptic curve cryptography or related mathematics.
// If Delta = 0, the curve is singular, meaning it has a point where the curve intersects itself or has a cusp. Such curves are not useful in elliptic curve cryptography because they lack the necessary group structure.
// In this context, the discriminant is a test for the curve's validity. A non-zero discriminant guarantees that the curve is suitable for cryptographic applications and other elliptic curve operations.
fn is_elliptic_curve(a: i32, b: i32, q: i32) -> bool {
    (4 * a.pow(3) + 27 * b.pow(2)) % q != 0
}

fn count_points(curve: &EllipticCurve, q: i32) -> i32 {
    let mut count = 1;
    for x in 0..q {
        let y_squared = (x.pow(3) + curve.a * x + curve.b) % q;
        for y in 0..q {
            if (y * y) % q == y_squared {
                count += 1;
            }
        }
    }
    count
}

pub fn run() {
    let q = 23;
    // [q + 1 − 2√q, q+1 + 2√q]=> Hasse bound
    let low = (q + 1) - (2.0 * (q as f64).sqrt()).floor() as i32;
    let high = (q + 1) + (2.0 * (q as f64).sqrt()).floor() as i32;
    let size = high - low;

    let mut rng = rand::thread_rng();
    let mut curves = Vec::new();
    let mut orders = HashSet::new();

    while curves.len() < (size + 1) as usize {
        let a = rng.gen_range(0..q);
        let b = rng.gen_range(0..q);
        if is_elliptic_curve(a, b, q) {
            let curve = EllipticCurve { a, b };
            let order = count_points(&curve, q);
            if !orders.contains(&order) {
                curves.push(curve.clone());
                orders.insert(order);
            }
        }
    }

    println!("Curves: {:?}", curves);
    println!("Orders: {:?}", orders);
}
