mod char_frob;
mod e_count_points;
mod e_over_f23_generalised;
mod e_over_finite_fields;
mod endomorphis_extension_field;
mod eover_q;
mod eover_real_field;
mod mul_by_m;
mod projective_point;

use char_frob::run as char_frob;
use e_count_points::run as e_count_points;
use e_over_f23_generalised::run as e_over_f23_generalised;
use e_over_finite_fields::run as e_over_finite_fields;
use endomorphis_extension_field::run as endomorphis_extension_field;
use eover_q::run as addElipticCurvePoints;
use eover_real_field::run as addOverRings;
use mul_by_m::run as mul_by_m;
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

    println!("\n P+Q using generalised formula.....");
    e_over_f23_generalised();

    println!("\n [m]P.....");
    mul_by_m();

    println!("\n Counting Points in a field...");
    e_count_points();

    println!("\n Extension fields.....");
    endomorphis_extension_field();

    println!("\n Charcater and Frobenius Map.....");
    char_frob();
}
