use std::ops::{Add, Sub};
use rand::prelude::*;

use crate::vector3::Vector3;

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Self {
        Point {
            x: _x,
            y: _y,
            z: _z,
        }
    }
    pub fn from(p: &Point) -> Point {
        Point {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
    pub fn new_random() -> Point  {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
    pub fn new_random_range(min: f64, max: f64) -> Point {
        let mut rng = rand::thread_rng();
        let x_ran: f64 = rng.gen();
        let y_ran: f64 = rng.gen();
        let z_ran: f64 = rng.gen();
        Point { 
            x: x_ran * (max - min) + min, 
            y: y_ran * (max - min) + min, 
            z: z_ran * (max - min) + min,
        }
    }
    pub fn random_in_unit_sphere()  -> Point {
        loop {
            let p = Point::new_random_range(-1.0, 1.0);
            let discriminant = p.x * p.x + p.y * p.y + p.z * p.z;
            if discriminant >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add<Vector3> for Point {
    type Output = Point;
    fn add(self, rhs: Vector3) -> Self::Output {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Point {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

impl Sub for Point {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
