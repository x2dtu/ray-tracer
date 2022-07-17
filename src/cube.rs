use std::{marker::PhantomData, rc::Rc};

use crate::{
    bounding_box::BoundingBox,
    hittable::{HitRecord, Hittable},
    hittable_vec::HittableVec,
    material::Material,
    point::Point,
    ray::Ray,
    rect::{Rect, RectType},
    utility::{self, INF},
    vector3::Vector3,
};

pub struct Cube<T: Material + 'static> {
    box_min: Point,
    box_max: Point,
    sides: HittableVec,
    sin_theta: f64,
    cos_theta: f64,
    _phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl<T: Material> Cube<T> {
    pub fn new(box_min: Point, box_max: Point, deg_rotation: f64, mat: Rc<T>) -> Cube<T> {
        let mut sides = HittableVec::new();

        sides.push(Box::new(Rect::new(
            box_min.x(),
            box_max.x(),
            box_min.y(),
            box_max.y(),
            box_max.z(),
            RectType::XYrect,
            Rc::clone(&mat),
        )));
        sides.push(Box::new(Rect::new(
            box_min.x(),
            box_max.x(),
            box_min.y(),
            box_max.y(),
            box_min.z(),
            RectType::XYrect,
            Rc::clone(&mat),
        )));

        sides.push(Box::new(Rect::new(
            box_min.x(),
            box_max.x(),
            box_min.z(),
            box_max.z(),
            box_max.y(),
            RectType::XZrect,
            Rc::clone(&mat),
        )));
        sides.push(Box::new(Rect::new(
            box_min.x(),
            box_max.x(),
            box_min.z(),
            box_max.z(),
            box_min.y(),
            RectType::XZrect,
            Rc::clone(&mat),
        )));

        sides.push(Box::new(Rect::new(
            box_min.y(),
            box_max.y(),
            box_min.z(),
            box_max.z(),
            box_max.x(),
            RectType::YZrect,
            Rc::clone(&mat),
        )));
        sides.push(Box::new(Rect::new(
            box_min.y(),
            box_max.y(),
            box_min.z(),
            box_max.z(),
            box_min.x(),
            RectType::YZrect,
            Rc::clone(&mat),
        )));

        let radians = utility::degrees_to_radians(deg_rotation);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        Cube {
            box_min,
            box_max,
            sides,
            sin_theta,
            cos_theta,
            _phantom: Default::default(),
        }
    }
}

impl<T: Material> Hittable for Cube<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let x = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        let z = self.sin_theta * r.direction().x() + self.cos_theta * r.origin().z();
        let origin = Point::new(x, r.origin().y(), z);
        let direction = Vector3::new(x, r.direction().y(), z);

        let rotated_r = Ray::new(origin, direction);

        if let Some(rec) = self.sides.hit(r, t_min, t_max) {
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
        let bbox = BoundingBox::new(self.box_min.clone(), self.box_max.clone());
        // if self.sin_theta == 0.0 {
        //     // then we have no rotation so just return default bbox
        //     return Some(bbox);
        // }
        let mut min = Point::new(INF, INF, INF);
        let mut max = Point::new(-INF, -INF, -INF);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let new_x = self.cos_theta * x + self.sin_theta * z;
                    let new_z = -self.sin_theta * x + self.cos_theta * z;

                    let tester = Vector3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        Some(BoundingBox::new(min, max))
    }
}
