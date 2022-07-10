use std::rc::Rc;

use crate::{material::Material, point::Point, ray::Ray, vector3::Vector3};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn new(
        point: Point,
        normal: Vector3,
        t: f64,
        front_face: bool,
        material: Rc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
    pub fn from(h: &HitRecord) -> HitRecord {
        HitRecord {
            point: h.point.clone(),
            normal: h.normal.clone(),
            t: h.t,
            front_face: h.front_face,
            material: Rc::clone(&h.material),
        }
    }
    pub fn clone(&self) -> Self {
        HitRecord {
            point: self.point.clone(),
            normal: self.normal.clone(),
            t: self.t,
            front_face: self.front_face,
            material: Rc::clone(&self.material),
        }
    }
    // pub fn point(&self) -> &Point {
    //     &self.p
    // }
    // pub fn normal(&self) -> &Vector3 {
    //     &self.normal
    // }
    // pub fn t(&self) -> f64 {
    //     self.t
    // }
    // pub fn front_face(&self) -> bool {
    //     self.front_face
    // }
    // pub fn material(&self) -> &Rc<dyn Material> {
    //     &self.material
    // }
    // pub fn set_point(&mut self, new: Point) {
    //     self.p = new;
    // }
    pub fn set_face_normal(r: &Ray, outward_normal: &Vector3) -> (Vector3, bool) {
        let front_face = Vector3::dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            Vector3::from(outward_normal)
        } else {
            -Vector3::from(outward_normal)
        };
        (normal, front_face)
    }
    // pub fn set_t(&mut self, new: f64) {
    //     self.t = new;
    // }
    // pub fn set_front_face(&mut self, new: bool) {
    //     self.front_face = new;
    // }
    // pub fn set_material(&mut self, new: Rc<dyn Material>) {
    //     self.material = new;
    // }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
