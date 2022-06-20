use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::{Add, Mul};

pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn write(&self, f: &mut BufWriter<File>) {
        let max = 255.999;
        writeln!(
            f,
            "{} {} {}",
            (max * self.red()) as i32,
            (max * self.green()) as i32,
            (max * self.blue()) as i32
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
