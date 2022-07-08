use crate::{hittable::HitRecord, ray::Ray, color::Color};

pub struct ScatterResult {
    pub success: bool,
    pub attenuation: Color,
    pub scattered: Ray,
}

impl ScatterResult {
    pub fn new(success: bool, attenuation: Color, scattered: Ray) -> ScatterResult {
        ScatterResult { success, attenuation, scattered }
    }
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult;
}