use crate::{color::Color, material::{Material, ScatterResult}, vector3::Vector3, ray::Ray, hittable::HitRecord};

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord<Self>) -> ScatterResult {
        let reflected = Vector3::reflect(&Vector3::unit_vector(r_in.direction()), rec.normal());
        let scattered = Ray::new(rec.point().clone(), reflected);
        let attenuation = self.albedo.clone();
        let success = Vector3::dot(scattered.direction(), rec.normal()) > 0.0;
        ScatterResult { success, attenuation, scattered }
    }
}