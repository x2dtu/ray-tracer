use crate::{point::Point, ray::Ray, vector3::Vector3};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector3,
    vertical: Vector3,
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Camera {
        let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = Point::origin()
            - horizontal.clone() / 2.0
            - vertical.clone() / 2.0
            - Vector3::new(0.0, 0.0, FOCAL_LENGTH);
        Camera {
            origin: Point::origin(),
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner.clone()
                + self.horizontal.clone() * u
                + self.vertical.clone() * v
                - self.origin.clone(),
        )
    }
}
