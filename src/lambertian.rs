use crate::{color::Color, material::{Material, ScatterResult}, ray::Ray, hittable::HitRecord, vector3::Vector3};

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Lambertian {
        Lambertian { albedo: Color::from(albedo) }
    }
}

impl Material for Lambertian {
    // fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> ScatterResult {
    //     let mut scatter_direction = Vector3::from(rec.normal()) + Vector3::random_unit_vector();
    //     if scatter_direction.near_zero() {
    //         scatter_direction = Vector3::from(rec.normal());
    //     }
    //     let scattered = Ray::new(Point::from(rec.point()), scatter_direction);
    //     let attenuation = self.albedo.clone();
    //     ScatterResult { success: true, attenuation, scattered }
    // }
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let mut scatter_direction = rec.normal.clone() + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone()
        }
        let scattered = Ray::new(rec.point.clone(), scatter_direction);
        let attenuation = self.albedo.clone();
        ScatterResult {success: true, attenuation, scattered }
    }
}