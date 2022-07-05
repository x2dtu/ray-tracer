use crate::{hittable::HitRecord, ray::Ray, color::Color};

pub trait Material {
    fn scatter(r_in: &Ray, rec: &mut HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}