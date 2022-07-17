use std::ops::{Add, Index, IndexMut, Sub};

use crate::vector3::Vector3;

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
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

impl Index<usize> for Point {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index Out of Bounds!"),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index Out of Bounds!"),
        }
    }
}
