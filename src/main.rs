
use std::fs::File;
use std::io::Write;

mod vec3;


fn main() {
  
    chapter_1_output_an_image();

    let mut test = vec3::Vec3::new();
    test.data[0] = 10f32;
    let mut test2 = vec3::Vec3::new();
    test2.data[0] = 20f32;
    test += test2;

    print!("vec3[0] {} \n", test.x());

}

fn chapter_1_output_an_image()
{
  let mut file = File::create("myFirstImage.ppm").expect("Unable to create file");

    let nx = 200;
    let ny = 200;
    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(first_line.as_bytes()).expect("Unable to read file");

    for j in num_iter::range_step(ny-1, -1, -1)
    {
        for i in 0..nx
        {           
            let r = (i as f32) / (nx as f32);
            let g = (j as f32) / (ny as f32);
            let b = 0.2;
            let ir = 255.99 * r;
            let ig = 255.99 * g;
            let ib = 255.99 * b;

            let current_color_line = format!("{} {} {}", (ir as i32), (ig as i32), (ib as i32));
            file.write_all(current_color_line.as_bytes()).expect("Unable to read file");
        }
    }

    println!("Output ready!");
    
}
