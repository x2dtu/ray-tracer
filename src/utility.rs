use rand::prelude::*;

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 1.0)
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    rand() * (max - min) + min
}

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn clamp(val: f64, min: f64, max: f64) -> f64 {
    // this function ensures that the passed in val is between the min and max params
    if val > max {
        return max;
    }
    if val < min {
        return min;
    }
    val
}
