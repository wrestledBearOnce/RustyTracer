
mod vec3;
mod ray;


use std::fs::File;
use std::io::Write;


use vec3::Vec3;
use ray::Ray;


fn main() {
  
    chapter_1_output_an_image();
    chapter_2_create_image_with_vec3();
    chapter_3_ray();
}

fn chapter_1_output_an_image()
{
    let mut file = File::create("myFirstImage.ppm").expect("Unable to create file");

    let nx = 200;
    let ny = 100;
    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(first_line.as_bytes()).expect("Unable to read file");

    for j in num_iter::range_step(ny-1, 0, -1)
    {
        for i in 0..nx
        {    
            let r = (i as f32) / (nx as f32);
            let g = (j as f32) / (ny as f32);
            let b = 0.2;
            let ir = 255.99 * r;
            let ig = 255.99 * g;
            let ib = 255.99 * b;

            let current_color_line = format!("{} {} {}\n", (ir as i32), (ig as i32), (ib as i32));
            file.write_all(current_color_line.as_bytes()).expect("Unable to read file");
        }
    }

    println!("Output ready!");
    
}

fn chapter_2_create_image_with_vec3()
{
    let mut file = File::create("mySecondImage.ppm").expect("Unable to create file");

    let nx = 200;
    let ny = 100;
    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(first_line.as_bytes()).expect("Unable to read file");

    for j in num_iter::range_step(ny-1, 0, -1)
    {
        for i in 0..nx
        {    
            let col = Vec3::ctor((i as f32) / (nx as f32), (j as f32) / (ny as f32), 0.2);
            
            let ir = 255.99 * col.x();
            let ig = 255.99 * col.y();
            let ib = 255.99 * col.z();

            let current_color_line = format!("{} {} {}\n", (ir as i32), (ig as i32), (ib as i32));
            file.write_all(current_color_line.as_bytes()).expect("Unable to read file");
        }
    }
}

fn color(ray:&Ray) -> Vec3
{
    let unit_dir_y = ray.direction().make_unit_vector().y();
    let t =  0.5 * (unit_dir_y + 1.0);
    Vec3::ctor(1f32, 1f32, 1f32).mul(1.0 - t) + Vec3::ctor(0.5, 0.7, 1.0).mul(t)
}

fn chapter_3_ray()
{
     let mut file = File::create("myThirdImage.ppm").expect("Unable to create file");

    let nx = 200;
    let ny = 100;
    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(first_line.as_bytes()).expect("Unable to read file");

    let lower_left_corner = Vec3::ctor(-2.0, -1.0, -1.0);
    let horizontal = Vec3::ctor(4f32, 0f32, 0f32);
    let vertical = Vec3::ctor(0f32, 2f32, 0f32);
    let origin = Vec3::ctor(0f32, 0f32, 0f32);

    for j in num_iter::range_step(ny-1, 0, -1)
    {
        for i in 0..nx
        {    
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);

            let ray = Ray::ctor(origin.copy(), lower_left_corner.copy() + horizontal.mul(u) + vertical.mul(v)); // TODO: try to avoid copy traits
            let col = color(&ray);
            
            let ir = 255.99 * col.x();
            let ig = 255.99 * col.y();
            let ib = 255.99 * col.z();

            let current_color_line = format!("{} {} {}\n", (ir as i32), (ig as i32), (ib as i32));
            file.write_all(current_color_line.as_bytes()).expect("Unable to read file");
        }
    }
}

