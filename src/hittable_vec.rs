use std::{rc::Rc, marker::PhantomData};

use crate::{hittable::{HitRecord, Hittable}, material::Material, ray::Ray};

pub struct HittableVec<T, M> where M: Material, T: Hittable<M> {
    _phantom: PhantomData<M>,
    objects: Vec<T>,
}

#[allow(dead_code)]
impl<T: Hittable<M>, M: Material> HittableVec<T, M> {
    pub fn new() -> HittableVec<T, M> {
        HittableVec { objects: vec![], _phantom: Default::default() }
    }
    pub fn from(v: Vec<T>) -> HittableVec<T, M> {
        HittableVec { objects: v, _phantom: Default::default() }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn push(&mut self, object: T) {
        self.objects.push(object);
    }
}
impl<T: Hittable<M>, M: Material> Hittable<M> for HittableVec<T, M> {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord<M>,
    ) -> bool {
        let mut temp_rec = HitRecord::default(Rc::clone(rec.material()));
        let mut has_hit = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                has_hit = true;
                closest_so_far = temp_rec.t();
                *rec = temp_rec.clone();
            }
        }
        return has_hit;
    }
}
