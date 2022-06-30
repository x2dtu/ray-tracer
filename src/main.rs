mod color;
mod hittable;
pub mod hittable_vec;
mod point;
mod ray;
mod sphere;
mod vector3;
use color::Color;
use hittable::{HitRecord, Hittable};
use point::Point;
use ray::Ray;
use std::fs::File;
use std::io::{BufWriter, Write};
use vector3::Vector3;

use crate::hittable_vec::HittableVec;
use crate::sphere::Sphere;

// image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

// camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const INF: f64 = f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

fn main() {
    // World
    let mut world = HittableVec::new();
    world.push(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // camera
    let origin = Point::origin();
    let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin
        - horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

    // create ppm file
    let file_name = "output.ppm";
    let f = File::create(file_name).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    // write to ppm file to render an image
    writeln!(f, "P3").expect("unable to write");
    writeln!(f, "{IMAGE_WIDTH} {IMAGE_HEIGHT}").expect("unable to write");
    writeln!(f, "255").expect("unable to write");

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("Scanlines remaining: {j}\r");
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                Point::origin(),
                lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v
                    - Point::origin(), // subtract a point to get back to a vector
            );
            let pixel_color = ray_color(&r, &world);
            pixel_color.write(&mut f);
        }
    }
    println!();
    println!("Done.");
}

fn ray_color<T>(r: &Ray, world: &T) -> Color
where
    T: Hittable,
{
    // let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r);
    // if t > 0.0 {
    //     let v = r.at(t) - Vector3::new(0.0, 0.0, -1.0) - Point::origin();
    //     let n = Vector3::unit_vector(&v);
    //     return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    // }
    // let unit_direction = Vector3::unit_vector(r.direction());
    // let t = 0.5 * (unit_direction.y() + 1.0);
    // Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, INF, &mut rec) {
        return (Color::new(1.0, 1.0, 1.0) + Vector3::from(rec.normal())) * 0.5;
    }
    let unit_direction = Vector3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let origin_to_center = r.origin().clone() - center.clone();
    // next solve quadratic formula
    let a = r.direction().length_squared();
    let b = 2.0 * Vector3::dot(&origin_to_center, r.direction());
    let c = origin_to_center.length_squared() - (radius * radius);
    let discriminant = b * b - 4.0 * a * c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    };
}
