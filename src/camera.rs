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
    u: Vector3,
    v: Vector3,
    lens_radius: f64,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vector3,
        vfov: f64, // vertical fov in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vector3::unit_vector(&(lookfrom.clone() - lookat.clone()));
        let u = Vector3::unit_vector(&Vector3::cross(&vup, &w));
        let v = Vector3::cross(&w, &u);

        let horizontal = u.clone() * viewport_width * focus_dist;
        let vertical = v.clone() * viewport_height * focus_dist;
        let lower_left_corner =
            lookfrom.clone() - horizontal.clone() / 2.0 - vertical.clone() / 2.0 - (w * focus_dist);

        let lens_radius = aperture / 2.0;

        Camera {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vector3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u.clone() * rd.x() + self.v.clone() * rd.y();

        Ray::new(
            self.origin.clone() + offset.clone(),
            self.lower_left_corner.clone()
                + self.horizontal.clone() * s
                + self.vertical.clone() * t
                - self.origin.clone()
                - offset,
        )
    }
}
