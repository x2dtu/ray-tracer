use crate::{point::Point, vector3::Vector3, ray::Ray};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector3,
    vertical: Vector3
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Camera {
        let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = Point::origin() - Vector3::from(&horizontal) / 2.0 - Vector3::from(&vertical) / 2.0 - Vector3::new(0.0, 0.0, FOCAL_LENGTH);
        Camera { origin: Point::origin(), lower_left_corner, horizontal, vertical }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(Point::from(&self.origin), Point::from(&self.lower_left_corner) + Vector3::from(&self.horizontal) * u + Vector3::from(&self.vertical) * v - Point::from(&self.origin))
    }
}

