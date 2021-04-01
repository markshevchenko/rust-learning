mod geo;

use geo::{Point, distance};

fn main() {
    let point1 = Point { x: 1.0, y: 2.0 };
    let point2 = Point { x: 2.0, y: 1.0 };

    println!("{}", distance(&point1, &point2));
}
