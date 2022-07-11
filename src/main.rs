mod camera;
mod color;
mod dielectric;
mod hittable;
pub mod hittable_vec;
mod lambertian;
mod material;
mod metal;
mod point;
mod ray;
mod sphere;
mod utility;
mod vector3;
use color::Color;
use dielectric::Dielectric;
use hittable::Hittable;
use lambertian::Lambertian;
use metal::Metal;
use point::Point;
use ray::Ray;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;
use utility::{rand, INF};
use vector3::Vector3;

use crate::camera::Camera;
use crate::hittable_vec::HittableVec;
use crate::sphere::Sphere;
// use crate::utility::PI;

// image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 10;
const MAX_DEPTH: i32 = 50;

fn main() {
    // World
    let world = random_scene();

    // camera
    let camera = create_camera();

    // create ppm file
    let file_name = "output.ppm";
    let f = File::create(file_name).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    // write to ppm file to render an image
    writeln!(f, "P3").expect("unable to write");
    writeln!(f, "{IMAGE_WIDTH} {IMAGE_HEIGHT}").expect("unable to write");
    writeln!(f, "255").expect("unable to write");

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\x1B[2J\x1B[1;1H");
        println!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color.write(&mut f, SAMPLES_PER_PIXEL as f64);
        }
    }
    println!();
    println!("Done Rendering! 😀");
}

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i32) -> Color {
    if depth <= 0 {
        // If this is true then at this point we have exceeded the ray bounce limit.
        // Since the light will never not hit the hittable object, we say no light is gathered.
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(x) = world.hit(r, 0.001, INF) {
        let scatter_result = x.material.scatter(r, &x);
        if scatter_result.success {
            return scatter_result.attenuation
                * ray_color(&scatter_result.scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = Vector3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

// fn create_world() -> HittableVec {
//     let mut world = HittableVec::new();

//     let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
//     let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
//     // let material_center = Rc::new(Dielectric::new(1.5)));
//     let material_left = Rc::new(Dielectric::new(1.5));
//     let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

//     world.push(Box::new(Sphere::new(
//         Point::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point::new(0.0, 0.0, -1.0),
//         0.5,
//         material_center,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point::new(-1.0, 0.0, -1.0),
//         0.5,
//         Rc::clone(&material_left),
//     )));
//     world.push(Box::new(Sphere::new(
//         Point::new(-1.0, 0.0, -1.0),
//         -0.45,
//         material_left,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point::new(1.0, 0.0, -1.0),
//         0.5,
//         material_right,
//     )));

//     world
// }

fn random_scene() -> HittableVec {
    let mut world = HittableVec::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utility::rand();
            let center = Point::new((a as f64) + 0.9*utility::rand(), 0.2, (b as f64) + 0.9*utility::rand());

            if (center.clone() - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.75 {
                    let albedo = Color::from_vector(Vector3::new_random()) * Color::from_vector(Vector3::new_random());
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
                else if choose_mat < 0.95 {
                    let albedo = Color::from_vector(Vector3::new_random_range(0.5, 1.0));
                    let fuzz = utility::rand_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
                else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
            
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

fn create_camera() -> Camera {
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    )
}
