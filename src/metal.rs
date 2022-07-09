use crate::{
    color::Color,
    hittable::HitRecord,
    material::{Material, ScatterResult},
    ray::Ray,
    vector3::Vector3,
};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

#[allow(dead_code)]
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let reflected = Vector3::reflect(&Vector3::unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray::new(
            rec.point.clone(),
            reflected + (Vector3::random_in_unit_sphere() * self.fuzz),
        );
        let attenuation = self.albedo.clone();
        let success = Vector3::dot(scattered.direction(), &rec.normal) >= 0.0;
        ScatterResult {
            success,
            attenuation,
            scattered,
        }
    }
}
