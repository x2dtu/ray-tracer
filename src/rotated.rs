use std::rc::Rc;

use crate::{
    bounding_box::BoundingBox,
    hittable::{HitRecord, Hittable},
    point::Point,
    ray::Ray,
    utility::{self, INF},
    vector3::Vector3,
};

pub struct Rotated<T: Hittable> {
    obj: T,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<BoundingBox>,
}

impl<T: Hittable> Rotated<T> {
    pub fn new(obj: T, deg_rotation: f64) -> Self {
        let radians = utility::degrees_to_radians(deg_rotation);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Point::new(INF, INF, INF);
        let mut max = Point::new(-INF, -INF, -INF);

        let bbox_option = obj.bounding_box();
        if let Some(bbox) = bbox_option {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                        let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                        let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vector3::new(new_x, y, new_z);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            let bounding_box = Some(BoundingBox::new(min, max));
            return Rotated {
                obj,
                sin_theta,
                cos_theta,
                bounding_box,
            };
        }
        Rotated {
            obj,
            sin_theta,
            cos_theta,
            bounding_box: None,
        }
    }
}

impl<T: Hittable> Hittable for Rotated<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let x = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        let z = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();
        let origin = Point::new(x, r.origin().y(), z);

        let x = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        let z = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();
        let direction = Vector3::new(x, r.direction().y(), z);

        let rotated_r = Ray::new(origin, direction);

        if let Some(rec) = self.obj.hit(r, t_min, t_max) {
            let point = Point::new(
                self.cos_theta * rec.point.x() + self.sin_theta * rec.point.z(),
                rec.point.y(),
                -self.sin_theta * rec.point.x() + self.cos_theta * rec.point.z(),
            );
            let outward_normal = Vector3::new(
                self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
                rec.normal.y(),
                -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
            );
            let (normal, front_face) = HitRecord::set_face_normal(&rotated_r, &outward_normal);
            let result = HitRecord {
                point,
                normal,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                front_face,
                material: Rc::clone(&rec.material),
            };
            return Some(result);
        }
        return None;
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        if let Some(bbox) = self.bounding_box.as_ref() {
            return Some(bbox.clone());
        }
        None
    }
}
