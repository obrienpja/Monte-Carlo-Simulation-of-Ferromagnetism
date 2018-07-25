use std::fmt;

pub struct Point{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Point{
    pub fn dot(self, second_point: Point) -> f64 {
        self.x * second_point.x + self.y * second_point.y + self.z * second_point.z
    }
}