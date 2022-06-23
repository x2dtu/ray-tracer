mod color;
mod point;
mod ray;
mod vector3;
use color::Color;
use point::Point;
use ray::Ray;
use std::fs::File;
use std::io::stdout;
use std::io::{BufWriter, Write};
use vector3::Vector3;

fn main() {
    // image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

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
        println!("Scanlines remaining: {j}");
        stdout().flush().expect("");
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                Point::origin(),
                lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v
                    - Point::origin(),
            );
            let pixel_color = ray_color(&r);
            pixel_color.write(&mut f);
        }
    }
    println!("Done.");
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = Vector3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> bool {
    let origin_to_center = r.origin().clone() - center.clone();
    // next solve quadratic formula
    let a = Vector3::dot(r.direction(), r.direction());
    let b = 2.0 * Vector3::dot(&origin_to_center, r.direction());
    let c = Vector3::dot(&origin_to_center, &origin_to_center) - (radius * radius);
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}
