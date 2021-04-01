use super::Point;

pub fn add(point1: Point, point2: Point) -> Point {
    Point { x: point1.x + point2.x, y: point1.y + point2.y }
}

pub fn scalar_mul(point1: Point, point2: Point) -> f64 {
    point1.x * point2.x + point1.y * point2.y
}