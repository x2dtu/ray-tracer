use crate::{point::Point, ray::Ray, utility::degrees_to_radians, vector3::Vector3};

// pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
// const VIEWPORT_HEIGHT: f64 = 2.0;
// const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
// const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector3,
    vertical: Vector3,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vector3,
        vfov: f64, // vertical fov in degrees
        aspect_ratio: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vector3::unit_vector(&(lookfrom.clone() - lookat.clone()));
        let u = Vector3::unit_vector(&Vector3::cross(&vup, &w));
        let v = Vector3::cross(&w, &u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner =
            lookfrom.clone() - horizontal.clone() / 2.0 - vertical.clone() / 2.0 - w;

        Camera {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner.clone()
                + self.horizontal.clone() * s
                + self.vertical.clone() * t
                - self.origin.clone(),
        )
    }
}
