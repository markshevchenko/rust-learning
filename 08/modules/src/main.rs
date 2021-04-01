mod geo;

use geo::{Point, distance};
use geo::math::{add, scalar_mul};

fn main() {
    let point1 = Point { x: 1.0, y: 2.0 };
    let point2 = Point { x: 2.0, y: 1.0 };

    println!("{}", distance(point1, point2));

    println!("{:?}", add(point1, point2));

    println!("{}", scalar_mul(point1, point2));
}
