use std::rc::Rc;

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

pub struct Cube {
    box_min: Point,
    box_max: Point,
    sides: HittableVec,
}

#[allow(dead_code)]
impl Cube {
    pub fn new(box_min: Point, box_max: Point, mat: Rc<dyn Material>) -> Cube {
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

        Cube {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox::new(self.box_min.clone(), self.box_max.clone()))
        // if self.sin_theta == 0.0 {
        //     // then we have no rotation so just return default bbox
        //     return Some(bbox);
        // }
    }
}
