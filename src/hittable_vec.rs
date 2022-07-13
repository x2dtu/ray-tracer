use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    bounding_box::BoundingBox,
};

pub struct HittableVec {
    objects: Vec<Box<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableVec {
    pub fn new() -> HittableVec {
        HittableVec { objects: vec![] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn push(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
impl Hittable for HittableVec {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(x) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = x.t;
                result = Some(x);
            }
        }
        result
    }
    fn bounding_box(&self) -> Option<BoundingBox> {        
        if self.objects.is_empty() {
            return None;
        }
        let mut result: Option<BoundingBox> = None;

        for object in &self.objects {
            if let Some(x) = object.bounding_box() {
                result = Some(BoundingBox::surrounding_box(&result.unwrap_or(x.clone()), &x));
            }
        }
        result
    }
}
