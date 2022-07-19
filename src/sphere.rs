use std::rc::Rc;

use crate::{
    bounding_box::BoundingBox,
    hittable::{HitRecord, Hittable},
    material::Material,
    point::Point,
    ray::Ray,
    utility::PI,
    vector3::Vector3,
};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
    pub fn center(&self) -> &Point {
        &self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }

    fn get_sphere_uv(&self, p: &Vector3) -> (f64, f64) {
        let theta = -p.clone().y().acos();
        let phi = -p.clone().z().atan2(p.clone().x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = r.origin().clone() - self.center.clone();
        let a = r.direction().length_squared();
        let half_b = Vector3::dot(&origin_to_center, r.direction());
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
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
        let (u, v) = self.get_sphere_uv(&outward_normal);
        let hit_record = HitRecord {
            point,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        };
        return Some(hit_record);
    }
    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            self.center.clone() - Vector3::new(self.radius, self.radius, self.radius),
            self.center.clone() + Vector3::new(self.radius, self.radius, self.radius),
        ))
    }
}
