use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableVec {
    objects: Vec<Box<dyn Hittable>>, // _phantom: PhantomData<M>,
                                     // objects: Vec<T>,
}

#[allow(dead_code)]
impl HittableVec {
    pub fn new() -> HittableVec {
        HittableVec { objects: vec![] } // _phantom: Default::default()
    }
    pub fn from(v: Vec<Box<dyn Hittable>>) -> HittableVec {
        HittableVec { objects: v } //_phantom: Default::default()
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
}
