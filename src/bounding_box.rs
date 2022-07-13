use crate::{point::Point, ray::Ray};

pub struct BoundingBox {
    minimum: Point,
    maximum: Point,
}

impl BoundingBox {
    pub fn new(minimum: Point, maximum: Point) -> BoundingBox {
        BoundingBox {
            minimum,
            maximum
        }
    }
    pub fn clone(&self) -> BoundingBox {
        BoundingBox { minimum: self.minimum.clone(), maximum: self.maximum.clone() }
    }
    pub fn min(&self) -> &Point {
        &self.minimum
    }
    pub fn max(&self) -> &Point {
        &self.maximum
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let t0 = (self.minimum[i] - r.origin()[i]) / r.direction()[i].min((self.maximum[i] - r.origin()[i]) / r.direction()[i]);
            let t1 = (self.minimum[i] - r.origin()[i]) / r.direction()[i].max((self.maximum[i] - r.origin()[i]) / r.direction()[i]);
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    pub fn area(&self) -> f64 {
        let l = self.maximum.x() - self.minimum.x();
        let w = self.maximum.y() - self.minimum.y();
        let h = self.maximum.z() - self.minimum.z();
        2.0 * (l * w + w * h + l * h)
    }
    pub fn longest_axis(&self) -> i32 {
        let l = self.maximum.x() - self.minimum.x();
        let w = self.maximum.y() - self.minimum.y();
        let h = self.maximum.z() - self.minimum.z();
        if l > w && l > h {
            return 0;
        }
        if w > h {
            return 1;
        }
        return 2;
    }
    pub fn surrounding_box(box0: &BoundingBox, box1: &BoundingBox) -> BoundingBox {
        let small = Point::new(box0.minimum.x().min(box1.minimum.x()), box0.minimum.y().min(box1.minimum.y()), box0.minimum.z().min(box1.minimum.z()));
        let big = Point::new(box0.minimum.x().max(box1.minimum.x()), box0.minimum.y().max(box1.minimum.y()), box0.minimum.z().max(box1.minimum.z()));
        BoundingBox { minimum: small, maximum: big }

    }
}