use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::{Add, AddAssign, Mul};

use crate::utility;
use crate::vector3::Vector3;

pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn from_vector(v: Vector3) -> Color {
        Color {
            red: v.x(),
            green: v.y(),
            blue: v.z()
        }
    }
    pub fn clone(&self) -> Self {
        Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
    pub fn write(&self, f: &mut BufWriter<File>, samples_per_pixel: f64) {
        let max = 256.0;
        let scale = 1.0 / samples_per_pixel;
        let r = utility::clamp((self.red() * scale).sqrt(), 0.0, 0.999);
        let g = utility::clamp((self.green() * scale).sqrt(), 0.0, 0.999);
        let b = utility::clamp((self.blue() * scale).sqrt(), 0.0, 0.999);
        writeln!(
            f,
            "{} {} {}",
            (max * r) as i32,
            (max * g) as i32,
            (max * b) as i32
        )
        .expect("unable to write");
    }
    pub fn red(&self) -> f64 {
        self.red
    }
    pub fn green(&self) -> f64 {
        self.green
    }
    pub fn blue(&self) -> f64 {
        self.blue
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, right: Color) -> Color {
        Color {
            red: self.red + right.red,
            green: self.green + right.green,
            blue: self.blue + right.blue,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.blue += rhs.blue;
        self.green += rhs.green;
    }
}

// interpret x, y, z of vector3 as a color for simplistic ray tracing
impl Add<Vector3> for Color {
    type Output = Color;
    fn add(self, rhs: Vector3) -> Self::Output {
        Color {
            red: self.red + rhs.x(),
            green: self.green + rhs.y(),
            blue: self.blue + rhs.z(),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}
