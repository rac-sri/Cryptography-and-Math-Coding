mod e_over_finite_fields;
mod eover_q;
mod eover_real_field;
mod projective_point;

use e_over_finite_fields::run as e_over_finite_fields;
use eover_q::run as addElipticCurvePoints;
use eover_real_field::run as addOverRings;
use projective_point::run as projective_point;

fn main() {
    println!("Some cryptography ....");
    println!("\nAdd Eliptic curve points...");
    addElipticCurvePoints();
    println!("\nAdd Eliptic curve points over infinite field...");
    addOverRings();
    println!("\nE over finite field of 11...");
    e_over_finite_fields();
    println!("\n Checking if two points in affine space are isomorphic in Projectile Space in field of 41....");
    projective_point();
}
