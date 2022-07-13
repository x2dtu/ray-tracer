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
    pub fn clone(&self) -> Self {
        HitRecord {
            point: self.point.clone(),
            normal: self.normal.clone(),
            t: self.t,
            front_face: self.front_face,
            material: Rc::clone(&self.material),
        }
    }
    pub fn set_face_normal(r: &Ray, outward_normal: &Vector3) -> (Vector3, bool) {
        let front_face = Vector3::dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
        (normal, front_face)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
