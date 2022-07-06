use std::{rc::Rc, cell::RefCell};

use crate::{point::Point, ray::Ray, vector3::Vector3, material::Material};

pub struct HitRecord<T: Material + ?Sized> {
    p: Point,
    normal: Vector3,
    t: f64,
    front_face: bool,
    material: Rc<RefCell<T>>
}

#[allow(dead_code)]
impl<T: Material> HitRecord<T> {
    pub fn new(p: Point, normal: Vector3, t: f64, front_face: bool, material: Rc<RefCell<T>>) -> HitRecord<T> {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material
        }
    }
    pub fn default(material: Rc<RefCell<T>>) -> HitRecord<T> {
        HitRecord {
            p: Point::origin(),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material
        }
    }
    pub fn from(h: &HitRecord<T>) -> HitRecord<T> {
        HitRecord {
            p: h.p.clone(),
            normal: h.normal.clone(),
            t: h.t,
            front_face: h.front_face,
            material: Rc::clone(&h.material)
        }
    }
    pub fn clone(&self) -> Self {
        HitRecord { p: self.p.clone(), normal: self.normal.clone(), t: self.t, front_face: self.front_face, material: Rc::clone(&self.material) }
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
    pub fn material(&self) -> &Rc<RefCell<T>> {
        &self.material
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
    pub fn set_material(&mut self, new: Rc<RefCell<T>>) {
        self.material = new;
    }
}

pub trait Hittable<T: Material> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<T>) -> bool;
}
