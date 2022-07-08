use crate::{
    color::Color,
    hittable::HitRecord,
    material::{Material, ScatterResult},
    ray::Ray,
    vector3::Vector3,
};

pub struct Lambertian {
    albedo: Color,
}

#[allow(dead_code)]
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let mut scatter_direction = rec.normal.clone() + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone()
        }
        let scattered = Ray::new(rec.point.clone(), scatter_direction);
        let attenuation = self.albedo.clone();
        ScatterResult {
            success: true,
            attenuation,
            scattered,
        }
    }
}
