pub mod math;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn distance(point1: Point, point2: Point) -> f64 {
    let delta_x = point1.x - point2.x;
    let delta_y = point1.y - point2.y;

    (delta_x * delta_x + delta_y * delta_y).sqrt()
}

#[test]
#[allow(non_snake_case)]
fn distance__with_1_distance__returns_1() {
    let actual = distance(Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 0.0 });

    assert_eq!(1.0, actual);
}