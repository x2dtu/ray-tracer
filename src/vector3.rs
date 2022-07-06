use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};
use crate::random::rand;

#[derive(Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Vector3 {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Vector3 {
        let [x, y, z] = [_x, _y, _z];
        Vector3 { x, y, z }
    }
    pub fn from(v: &Vector3) -> Vector3 {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
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
    pub fn new_random() -> Vector3  {
        Vector3 {
            x: rand(),
            y: rand(),
            z: rand(),
        }
    }
    pub fn new_random_range(min: f64, max: f64) -> Vector3 {
        Vector3 { 
            x: rand() * (max - min) + min, 
            y: rand() * (max - min) + min, 
            z: rand() * (max - min) + min,
        }
    }
    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let v = Vector3::new_random_range(-1.0, 1.0);
            if v.length_squared() >= 1.0 {
                continue;
            }
            return v;
        }
    }
    pub fn random_unit_vector() -> Vector3 {
        Vector3::unit_vector(&Vector3::random_in_unit_sphere())
    }
    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let in_unit_sphere = Vector3::random_in_unit_sphere();
        if Vector3::dot(&in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        }
        -in_unit_sphere
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot(left: &Vector3, right: &Vector3) -> f64 {
        (left.x * right.x) + (left.y * right.y) + (left.z * right.z)
    }
    pub fn cross(left: &Vector3, right: &Vector3) -> Vector3 {
        Vector3 {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x,
        }
    }
    pub fn unit_vector(v: &Vector3) -> Vector3 {
        let copy = Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        };
        let length = copy.length();
        copy / length
    }
    pub fn clone(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
    pub fn near_zero(&self) -> bool {
        // Returns true if this vector is close to 0 in all its dimensions
        let delta = 1e-8;
        self.x.abs() < delta && self.y.abs() < delta && self.z.abs() < delta
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, right: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + right.x,
            y: self.y + right.y,
            z: self.z + right.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, right: Vector3) {
        self.x *= right.x;
        self.y *= right.y;
        self.z *= right.z;
    }
}
impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}
