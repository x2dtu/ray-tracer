mod bounding_box;
mod camera;
mod color;
mod cube;
mod dielectric;
mod hittable;
mod hittable_vec;
mod lambertian;
mod material;
mod metal;
mod point;
mod ray;
mod rect;
mod sphere;
mod utility;
mod vector3;
use camera::Camera;
use color::Color;
use crossterm::{cursor, terminal, QueueableCommand};
use cube::Cube;
use dielectric::Dielectric;
use hittable::Hittable;
use hittable_vec::HittableVec;
use lambertian::Lambertian;
use metal::Metal;
use point::Point;
use ray::Ray;
use sphere::Sphere;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;
use utility::{rand, INF};
use vector3::Vector3;

// image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 5;
const MAX_DEPTH: i32 = 50;

fn main() {
    // World
    let world = random_scene();

    // camera
    let camera = create_camera();

    // create ppm file
    let cmd_line_args: Vec<String> = env::args().collect();
    let file_name = if cmd_line_args.len() == 2 {
        &cmd_line_args[1]
    } else {
        "output.ppm"
    };
    let f = File::create(file_name).expect("Unable to create file");
    let mut f = io::BufWriter::new(f);

    // write to ppm file to render an image
    writeln!(f, "P3").unwrap();
    writeln!(f, "{IMAGE_WIDTH} {IMAGE_HEIGHT}").unwrap();
    writeln!(f, "255").unwrap();

    println!();
    let mut stdout = io::stdout();
    // stdout.execute(cursor::Hide).unwrap(); // hide terminal cursor

    for j in (0..IMAGE_HEIGHT).rev() {
        // save terminal cursor position so we can go back to it
        stdout.queue(cursor::SavePosition).unwrap();
        stdout
            .write_all(format!("Scanlines remaining: {j}\n").as_bytes())
            .unwrap();

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

        // manipulate terminal cursor so that we print scanlines remaining on same line
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
    }
    println!("Done Rendering! 😀");
    // stdout.execute(cursor::Show).unwrap(); // show terminal cursor again
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

fn random_scene() -> HittableVec {
    let mut world = HittableVec::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let spheres = create_big_spheres();

    const RADIUS: f64 = 0.2;
    const CUBE_WIDTH: f64 = RADIUS * 1.5;

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utility::rand();
            let center = Point::new(
                (a as f64) + 0.6 * utility::rand(),
                RADIUS,
                (b as f64) + 0.6 * utility::rand(),
            );
            // let (deg_change_x, deg_change_z, deg_rotation) = random_rotation();
            let (deg_change_x, deg_change_z, deg_rotation) = (0.0, 0.0, 0.0);
            let cube_bottom = Point::new(
                center.x() - CUBE_WIDTH / 2.0 + deg_change_x,
                0.0,
                center.z() - CUBE_WIDTH / 2.0 + deg_change_z,
            );

            if (center.clone() - Point::new(4.0, 0.2, 0.0)).length() > 0.9
                && not_intersecting_with_big_spheres(&center, RADIUS, &spheres)
            {
                if choose_mat < 0.60 {
                    let albedo = Color::from_vector(Vector3::new_random())
                        * Color::from_vector(Vector3::new_random());
                    let material = Rc::new(Lambertian::new(albedo));
                    let b: Box<dyn Hittable> = if utility::rand() < 0.5 {
                        Box::new(Cube::new(
                            cube_bottom.clone(),
                            cube_bottom + Point::new(CUBE_WIDTH, CUBE_WIDTH, CUBE_WIDTH),
                            deg_rotation,
                            material,
                        ))
                    } else {
                        Box::new(Sphere::new(center, RADIUS, material))
                    };
                    world.push(b);
                } else if choose_mat < 0.85 {
                    let albedo = Color::from_vector(Vector3::new_random_range(0.5, 1.0));
                    let fuzz = utility::rand_range(0.0, 0.25);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    let b = Box::new(Sphere::new(center, RADIUS, material));
                    world.push(b);
                } else {
                    let material = Rc::new(Dielectric::new(1.5));
                    let b: Box<dyn Hittable> = if utility::rand() < 0.5 {
                        Box::new(Cube::new(
                            cube_bottom.clone(),
                            cube_bottom + Point::new(CUBE_WIDTH, CUBE_WIDTH, CUBE_WIDTH),
                            deg_rotation,
                            material,
                        ))
                    } else {
                        Box::new(Sphere::new(center, RADIUS, material))
                    };
                    world.push(b);
                }
            }
        }
    }

    for sphere in spheres {
        world.push(Box::new(sphere));
    }

    world
}

fn create_big_spheres() -> Vec<Sphere> {
    let material1 = Rc::new(Dielectric::new(1.5));
    let glass_sphere = Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1);

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let lambert_sphere = Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2);

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let metal_sphere = Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3);
    vec![glass_sphere, lambert_sphere, metal_sphere]
}

fn not_intersecting_with_big_spheres(center: &Point, radius: f64, spheres: &Vec<Sphere>) -> bool {
    // returns false if the sphere constructed through the provided center and radius intersects with any of the spheres in the provided sphere vector
    let mut result = true;
    for sphere in spheres {
        let dist_sq = (center.x() - sphere.center().x()) * (center.x() - sphere.center().x())
            + (center.z() - sphere.center().z()) * (center.z() - sphere.center().z());
        let rad_sum_sq = (radius + sphere.radius()) * (radius + sphere.radius());
        result = result && dist_sq > rad_sum_sq;
        // let (sphere_x_range, sphere_z_range) = get_sphere_ranges(&sphere);
        // let in_sphere_x = in_range(center.x() - radius, sphere_x_range)
        //     || in_range(center.x() + radius, sphere_x_range);
        // let in_sphere_z = in_range(center.z() - radius, sphere_z_range)
        //     || in_range(center.z() + radius, sphere_z_range);
        // result = result && !(in_sphere_x || in_sphere_z);
    }
    result
}

// fn get_sphere_ranges(sphere: &Sphere) -> ((f64, f64), (f64, f64)) {
//     (
//         (
//             sphere.center().x() - sphere.radius(),
//             sphere.center().x() + sphere.radius(),
//         ),
//         (
//             sphere.center().z() - sphere.radius(),
//             sphere.center().z() + sphere.radius(),
//         ),
//     )
// }

// fn in_range(p: f64, min_max: (f64, f64)) -> bool {
//     let (min, max) = min_max;
//     p >= min && p <= max
// }

// fn random_rotation() -> (f64, f64, f64) {
//     let possible_rotations = [-40, -30, -20, -10, 0];
//     let rand_position = (utility::rand() * possible_rotations.len() as f64) as usize;
//     let rotation = possible_rotations[rand_position];
//     if rotation == 0 {
//         return (0.0, 0.0, 0.0);
//     }
//     (
//         0.0225 * rotation as f64 - 0.2,
//         0.0425 * rotation as f64 + 0.15,
//         rotation as f64,
//     )
// }

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
