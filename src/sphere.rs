use std::{rc::Rc, cell::RefCell};

use crate::{point::Point, vector3::Vector3, material::Material, hittable::{Hittable, HitRecord}, ray::Ray};

pub struct Sphere<T: Material> {
    center: Point,
    radius: f64,
    material: Rc<RefCell<T>>
}

#[allow(dead_code)]
impl<T: Material> Sphere<T> {
    pub fn new(center: Point, radius: f64, material: Rc<RefCell<T>>) -> Sphere<T> {
        Sphere { center, radius, material }
    }
}

impl<T: Material> Hittable<T> for Sphere<T> {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord<T>,
    ) -> bool {

        let origin_to_center = r.origin().clone() - self.center.clone();
        let a = r.direction().length_squared();
        let b = 2.0 * Vector3::dot(&origin_to_center, r.direction());
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }

        // Find the nearest root that lies in the acceptable range
        let root = (-b - discriminant.sqrt()) / (2.0 * a);
        if root < t_min || root > t_max {
            // then use other root
            let root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root < t_min || root > t_max {
                // then both roots are not in acceptable range, so return false
                return false;
            }
        }

        // update hit record fields
        rec.set_t(root);
        rec.set_point(r.at(rec.t()));
        let outward_normal = (Point::from(rec.point()) - self.center.clone()) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.set_material(Rc::clone(&self.material));

        return true;
    }
}
