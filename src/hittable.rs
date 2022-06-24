use crate::{point::Point, ray::Ray, vector3::Vector3};

pub struct HitRecord {
    p: Point,
    normal: Vector3,
    t: f64,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn point(&self) -> &Point {
        &self.p
    }
    pub fn normal(&self) -> &Vector3 {
        &self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn set_point(&mut self, new: Point) {
        self.p = new;
    }
    pub fn set_normal(&mut self, new: Vector3) {
        self.normal = new;
    }
    pub fn set_t(&mut self, new: f64) {
        self.t = new;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
