use crate::{point::Point, vector3::Vector3};

pub struct Ray {
    origin: Point,
    direction: Vector3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(orig: Point, dir: Vector3) -> Ray {
        Ray {
            origin: orig,
            direction: dir,
        }
    }
    pub fn origin(&self) -> &Point {
        &self.origin
    }
    pub fn at(&self, t: f64) -> Point {
        let orig = Point::new(self.origin.x(), self.origin.y(), self.origin.z());
        let dir = Vector3::new(self.direction.x(), self.direction.y(), self.direction.z());
        orig + dir * t
    }
}
