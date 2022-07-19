use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub struct ScatterResult {
    pub success: bool,
    pub attenuation: Color,
    pub scattered: Ray,
}

#[allow(dead_code)]
impl ScatterResult {
    pub fn new(success: bool, attenuation: Color, scattered: Ray) -> ScatterResult {
        ScatterResult {
            success,
            attenuation,
            scattered,
        }
    }
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult;
}
