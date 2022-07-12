use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    point::Point,
    ray::Ray,
    vector3::Vector3,
};

pub struct Sphere<T: Material + 'static> {
    center: Point,
    radius: f64,
    material: Rc<T>,
}

#[allow(dead_code)]
impl<T: Material> Sphere<T> {
    pub fn new(center: Point, radius: f64, material: Rc<T>) -> Sphere<T> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<T: Material> Hittable for Sphere<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = r.origin().clone() - self.center.clone();
        let a = r.direction().length_squared();
        let half_b = Vector3::dot(&origin_to_center, r.direction());
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        // create hit record result
        let t = root;
        let point = r.at(t);
        let outward_normal = (point.clone() - self.center.clone()) / self.radius;
        let (normal, front_face) = HitRecord::set_face_normal(r, &outward_normal);
        let material = Rc::clone(&self.material);
        let hit_record = HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        };
        return Some(hit_record);
    }
}
