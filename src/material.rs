use crate::{hittable::HitRecord, ray::Ray, color::Color};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord<Self>, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}