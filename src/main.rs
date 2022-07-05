mod color;
mod hittable;
pub mod hittable_vec;
mod point;
mod ray;
mod sphere;
mod vector3;
mod camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use point::Point;
use ray::Ray;
use std::fs::File;
use std::io::{BufWriter, Write};
use vector3::Vector3;

use crate::camera::{Camera, ASPECT_RATIO};
use crate::hittable_vec::HittableVec;
use crate::sphere::Sphere;

// image
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

const INF: f64 = f64::INFINITY;
const PI: f64 = std::f64::consts::PI;
const MAX_DEPTH: i32 = 50;

fn main() {
    // World
    let mut world = HittableVec::new();
    world.push(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // camera
    let camera = Camera::new();

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
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color.write(&mut f, SAMPLES_PER_PIXEL as f64);
        }
    }
    println!();
    println!("Done.");
}

fn ray_color<T>(r: &Ray, world: &T, depth: i32) -> Color
where
    T: Hittable,
{
    if depth <= 0 {
        // If this is true then at this point we have exceeded the ray bounce limit.
        // Since the light will never not hit the hittable object, we say no light is gathered.
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, INF, &mut rec) {
        let target = Point::from(rec.point()) + Vector3::from(rec.normal()) + Point::random_in_unit_sphere();
        return ray_color(&Ray::new(Point::from(rec.point()), target - Point::from(rec.point())), world, depth - 1) * 0.5;
    }
    let unit_direction = Vector3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

// fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
//     let origin_to_center = r.origin().clone() - center.clone();
//     // next solve quadratic formula
//     let a = r.direction().length_squared();
//     let b = 2.0 * Vector3::dot(&origin_to_center, r.direction());
//     let c = origin_to_center.length_squared() - (radius * radius);
//     let discriminant = b * b - 4.0 * a * c;
//     return if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-b - discriminant.sqrt()) / (2.0 * a)
//     };
// }
