mod EoverQ;
mod EoverRealField;

use EoverQ::run as addElipticCurvePoints;
use EoverRealField::run as addOverRings;

fn main() {
    println!("Some cryptography ....");
    addElipticCurvePoints();
    addOverRings();
}
