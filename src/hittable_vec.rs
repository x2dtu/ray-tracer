use crate::hittable::{HitRecord, Hittable};

pub struct HittableVec<T: Hittable> {
    objects: Vec<T>,
}

#[allow(dead_code)]
impl<T: Hittable> HittableVec<T> {
    pub fn new() -> HittableVec<T> {
        HittableVec { objects: vec![] }
    }
    pub fn from(v: Vec<T>) -> HittableVec<T> {
        HittableVec { objects: v }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn push(&mut self, object: T) {
        self.objects.push(object);
    }
}
impl<T: Hittable> Hittable for HittableVec<T> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut has_hit = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                has_hit = true;
                closest_so_far = temp_rec.t();
                *rec = HitRecord::from(&temp_rec);
            }
        }
        return has_hit;
    }
}
