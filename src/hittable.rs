use crate::{point::Point, ray::Ray, vector3::Vector3};

pub struct HitRecord {
    p: Point,
    normal: Vector3,
    t: f64,
    front_face: bool,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn new(p: Point, normal: Vector3, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
    pub fn default() -> HitRecord {
        HitRecord {
            p: Point::origin(),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn from(h: &HitRecord) -> HitRecord {
        HitRecord {
            p: Point::from(&h.p),
            normal: Vector3::from(&h.normal),
            t: h.t,
            front_face: h.front_face,
        }
    }
    pub fn point(&self) -> &Point {
        &self.p
    }
    pub fn normal(&self) -> &Vector3 {
        &self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn set_point(&mut self, new: Point) {
        self.p = new;
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        self.front_face = Vector3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            Vector3::from(outward_normal)
        } else {
            -Vector3::from(outward_normal)
        };
    }
    pub fn set_t(&mut self, new: f64) {
        self.t = new;
    }
    pub fn set_front_face(&mut self, new: bool) {
        self.front_face = new;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
