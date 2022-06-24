use crate::{hittable::Hittable, point::Point, vector3::Vector3};

pub struct Sphere {
    center: Point,
    radius: f64,
}

#[allow(dead_code)]
impl Sphere {
    fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
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

        rec.set_t(root);
        rec.set_point(r.at(rec.t()));
        rec.set_normal((rec.point().clone() - self.center.clone()) / self.radius);

        return true;
    }
}
