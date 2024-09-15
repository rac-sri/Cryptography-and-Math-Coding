mod EoverQ;
mod EoverRealField;
mod e_over_finite_fields;

use e_over_finite_fields::run as e_over_finite_fields;
use EoverQ::run as addElipticCurvePoints;
use EoverRealField::run as addOverRings;

fn main() {
    println!("Some cryptography ....");
    println!("\nAdd Eliptic curve points...");
    addElipticCurvePoints();
    println!("\nAdd Eliptic curve points over infinite field...");
    addOverRings();
    println!("\nE over finite field of 11...");
    e_over_finite_fields();
}
