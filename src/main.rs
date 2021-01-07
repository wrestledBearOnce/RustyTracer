extern crate nalgebra as na;

mod image;
mod mesh;
mod ray;
mod sphere;

use crate::image::Image;
use crate::mesh::Mesh;
use crate::ray::Ray;
use crate::sphere::Sphere;
use na::Vector3;
use std::io::{stdout, Write};

fn main() {
    let s = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    // image related stuff
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 1080;
    let img_height = (img_width as f32 / aspect_ratio) as usize;
    let mut img = Image::new(img_width, img_height);

    // viewport related stuff
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    let mut stdout = stdout();

    for y in (0..img_height).rev() {
        print!("\rScanlines remaining: {}", y);
        stdout.flush().unwrap();

        for x in 0..img_width {
            let u = x as f32 / (img_width as f32 - 1.0);
            let v = y as f32 / (img_height as f32 - 1.0);

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let mut pixel_color = ray.color();
            let intersection_pt = s.intersects(&ray);
            if  intersection_pt > 0.0 {
                // surface normal
                let normal = (ray.at(intersection_pt) - Vector3::new(0.0,0.0,-1.0)).normalize_mut();
                pixel_color = normal * Vector3::new(0.25, 0.75, 0.5);
            }

            img.data.push(pixel_color);
        }
    }

    // one empty line due to stdout.flush() '\r' overwrite
    println!("");

    img.save("test.ppm");
}
