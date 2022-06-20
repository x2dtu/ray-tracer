use std::ops::Add;

use crate::vector3::Vector3;

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Self {
        Point {
            x: _x,
            y: _y,
            z: _z,
        }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Add<Vector3> for Point {
    type Output = Point;
    fn add(self, rhs: Vector3) -> Self::Output {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}
