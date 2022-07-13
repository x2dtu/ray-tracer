use std::rc::Rc;

use crate::{material::Material, hittable::{Hittable, HitRecord}, ray::Ray, vector3::Vector3, bounding_box::BoundingBox, point::Point };

pub struct Rect<T: Material + 'static> {
    material: Rc<T>,
    a0: f64, // for XYrect you could think of a0=x0, a1=x1, b0=y0, b1=y1
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    rect_type: RectType
}

#[allow(dead_code)]
pub enum RectType {
    XYrect,
    XZrect,
    YZrect,
}

#[allow(dead_code)]
impl<T: Material> Rect<T> {
    pub fn new(a0: f64, a1: f64,
        b0: f64,
        b1: f64,
        k: f64,
        rect_type: RectType,
        material: Rc<T>) -> Rect<T> {
            Rect { material, a0, a1, b0, b1, k, rect_type }
        }
}

impl<T: Material> Hittable for Rect<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.rect_type {
            RectType::XYrect => {
                let t = (self.k - r.origin().z()) / r.direction().z();
                if t < t_min || t > t_max {
                    return None;
                }
                let x = r.origin().x() + t * r.direction().x();
                let y = r.origin().y() + t * r.direction().y();
                if x < self.a0 || x > self.a1 || y < self.b0 || y > self.b1 {
                    return None;
                }
                let u = (x - self.a0) / (self.a1 - self.a0);
                let v = (y - self.b0) / (self.b1 - self.b0);
                let outward_normal = Vector3::new(0.0, 0.0, 1.0);
                let (normal, front_face) = HitRecord::set_face_normal(r, &outward_normal);
                let material = Rc::clone(&self.material);
                let point = r.at(t);
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
            RectType::XZrect => {
                let t = (self.k - r.origin().y()) / r.direction().y();
                if t < t_min || t > t_max {
                    return None;
                }
                let x = r.origin().x() + t * r.direction().x();
                let z = r.origin().z() + t * r.direction().z();
                if x < self.a0 || x > self.a1 || z < self.b0 || z > self.b1 {
                    return None;
                }
                let u = (x - self.a0) / (self.a1 - self.a0);
                let v = (z - self.b0) / (self.b1 - self.b0);
                let outward_normal = Vector3::new(0.0, 1.0, 0.0);
                let (normal, front_face) = HitRecord::set_face_normal(r, &outward_normal);
                let material = Rc::clone(&self.material);
                let point = r.at(t);
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
            RectType::YZrect => {
                let t = (self.k - r.origin().x()) / r.direction().x();
                if t < t_min || t > t_max {
                    return None;
                }
                let y = r.origin().y() + t * r.direction().y();
                let z = r.origin().z() + t * r.direction().z();
                if y < self.a0 || y > self.a1 || z < self.b0 || z > self.b1 {
                    return None;
                }
                let u = (y - self.a0) / (self.a1 - self.a0);
                let v = (z - self.b0) / (self.b1 - self.b0);
                let outward_normal = Vector3::new(1.0, 0.0, 0.0);
                let (normal, front_face) = HitRecord::set_face_normal(r, &outward_normal);
                let material = Rc::clone(&self.material);
                let point = r.at(t);
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
        }
    }
    fn bounding_box(&self) -> Option<BoundingBox> {
        const TOLERANCE: f64 = 0.0001;
        match self.rect_type {
            RectType::XYrect => {
                Some(BoundingBox::new(Point::new(self.a0, self.b0, self.k - TOLERANCE), 
                    Point::new(self.a1, self.b1, self.k + TOLERANCE)))
            }
            RectType::XZrect => {
                Some(BoundingBox::new(Point::new(self.a0, self.k - TOLERANCE, self.b0 ), 
                Point::new(self.a1, self.k + TOLERANCE, self.b1)))
            }
            RectType::YZrect => {
                Some(BoundingBox::new(Point::new(self.k - TOLERANCE, self.a0, self.b0 ), 
                Point::new(self.k + TOLERANCE, self.a1, self.b1)))
            }
        }
    }
}