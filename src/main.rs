mod vector3;
use std::fs::File;
use std::io::{BufWriter, Write};
use vector3::Vector3;

use crate::vector3::Color;

fn main() {
    // image
    let image_width = 256;
    let image_height = 256;

    // create ppm file
    let file_name = "output.ppm";
    let f = File::create(file_name).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    // write to ppm file to render an image
    writeln!(f, "P3").expect("unable to write");
    writeln!(f, "{image_width} {image_height}").expect("unable to write");
    writeln!(f, "255").expect("unable to write");

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {j}");
        for i in 0..image_width {
            let pixel_color = Vector3::new(
                (i as f64) / (image_width - 1) as f64,
                (j as f64) / (image_height - 1) as f64,
                0.25,
            );
            pixel_color.write_color(&mut f);
        }
    }
    println!("Done.");
}
