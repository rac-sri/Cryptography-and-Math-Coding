use crate::e_over_finite_fields::{inverse_mod, EllipticCurve, Fq, Point};

#[derive(Copy, Clone, Debug)]
struct ProjectivePoint {
    x: Fq,
    y: Fq,
    z: Fq,
}

const FIELD: u8 = 41;
impl ProjectivePoint {
    fn new(x: Fq, y: Fq, z: Fq) -> Self {
        ProjectivePoint { x, y, z }
    }

    fn is_isomorphic(&self, other: &ProjectivePoint) -> bool {
        // Check if both points are at infinity
        if self.z == Fq::new(0, FIELD) && other.z == Fq::new(0, FIELD) {
            return true; // Both points are at infinity, hence isomorphic
        }

        // Ensure other point is not at infinity
        if self.z == Fq::new(0, FIELD) || other.z == Fq::new(0, FIELD) {
            return false; // One point at infinity, not isomorphic
        }

        // Compare the ratios of the coordinates
        let x_ratio = self.x * inverse_mod(other.x.value, 41);
        let y_ratio = self.y * inverse_mod(other.y.value, 41);
        let z_ratio = self.z * inverse_mod(other.z.value, 41);

        // The points are isomorphic if all ratios are equal
        x_ratio == y_ratio && y_ratio == z_ratio
    }
}

pub fn run() {
    let field = 41;
    let p1 = Point {
        x: Fq::new(1, field),
        y: Fq::new(2, field),
    };

    let p2 = Point {
        x: Fq::new(5, field),
        y: Fq::new(6, field),
    };

    let eq = EllipticCurve::new(Fq::new(4, field), Fq::new(-1, field), field); // E: y^2 = x^3 + ax + b => y^2 = x^3 + 4x -1
    let result = eq.add(&p1, &p2);
    println!("Result of P1 + P2: {:?}", result);

    // Projective points
    let p_proj1 = ProjectivePoint::new(Fq::new(1, field), Fq::new(2, field), Fq::new(1, field));
    let p_proj2 = ProjectivePoint::new(Fq::new(5, field), Fq::new(6, field), Fq::new(1, field));

    println!(
        "P_proj1 and P_proj2 are isomorphic: {}. Point 1: {:?} Point 2: {:?}",
        p_proj1.is_isomorphic(&p_proj2),
        p_proj1,
        p_proj2
    );

    let p_proj1 = ProjectivePoint::new(Fq::new(1, field), Fq::new(2, field), Fq::new(1, field));
    let p_proj2 = ProjectivePoint::new(Fq::new(5, field), Fq::new(10, field), Fq::new(5, field));

    println!(
        "P_proj1 and P_proj2 are isomorphic: {}. Point 1: {:?} Point 2: {:?}",
        p_proj1.is_isomorphic(&p_proj2),
        (p_proj1.x.value, p_proj1.y.value, p_proj1.z.value),
        (p_proj2.x.value, p_proj2.y.value, p_proj2.z.value)
    );
}
