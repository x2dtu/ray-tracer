use std::{rc::Rc, marker::PhantomData};

use crate::{point::Point, hittable_vec::HittableVec, material::Material, rect::{Rect, RectType}, hittable::{Hittable, HitRecord}, ray::Ray, bounding_box::BoundingBox};

pub struct Cube<T: Material + 'static> {
    box_min: Point,
    box_max: Point,
    sides: HittableVec,
    _phantom: PhantomData<T>
}

#[allow(dead_code)]
impl<T: Material> Cube<T> {
    pub fn new(box_min: Point, box_max: Point, mat: Rc<T>) -> Cube<T> {
        let mut sides = HittableVec::new();

        sides.push(Box::new(Rect::new(box_min.x(), box_max.x(), box_min.y(), box_max.y(), box_max.z(), RectType::XYrect, Rc::clone(&mat))));
        sides.push(Box::new(Rect::new(box_min.x(), box_max.x(), box_min.y(), box_max.y(), box_min.z(), RectType::XYrect, Rc::clone(&mat))));

        sides.push(Box::new(Rect::new(box_min.x(), box_max.x(), box_min.z(), box_max.z(), box_max.y(), RectType::XZrect, Rc::clone(&mat))));
        sides.push(Box::new(Rect::new(box_min.x(), box_max.x(), box_min.z(), box_max.z(), box_min.y(), RectType::XZrect, Rc::clone(&mat))));

        sides.push(Box::new(Rect::new(box_min.y(), box_max.y(), box_min.z(), box_max.z(), box_max.x(), RectType::YZrect, Rc::clone(&mat))));
        sides.push(Box::new(Rect::new(box_min.y(), box_max.y(), box_min.z(), box_max.z(), box_min.x(), RectType::YZrect, Rc::clone(&mat))));

        Cube { box_min, box_max, sides, _phantom: Default::default() }
    }
}

impl<T: Material> Hittable for Cube<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox::new(self.box_min.clone(), self.box_max.clone()))
    }
}