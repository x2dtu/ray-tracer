use crate::{color::Color, material::Material, ray::Ray, hittable::HitRecord, vector3::Vector3, point::Point};

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Lambertian {
        Lambertian { albedo: Color::from(albedo) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord<Self>, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = Vector3::from(rec.normal()) + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = Vector3::from(rec.normal());
        }
        *scattered = Ray::new(Point::from(rec.point()), scatter_direction);
        *attenuation = self.albedo;
        true
    }
}