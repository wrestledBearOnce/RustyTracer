extern crate nalgebra as na;

use std::io::{Write, stdout};
use na::Vector3;


pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vector3<f32>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        return Image {
            width: width,
            height: height,
            data: Vec::new(),
        };
    }

    // save image, dest with extension
    pub fn save(&self, dest: &str) {

        println!("Saving image to {}", dest);

        let mut img =
            std::fs::File::create(dest).expect(&format!("creation of image {0} failed", dest));

        img.write_all(format!("P3\n{0} {1}\n255\n", self.width, self.height).as_bytes())
            .expect(&format!("writing of image header {} failed", dest));

        let mut stdout = stdout();

        let mut img_payload = "".to_owned();

        for (idx, d) in self.data.iter().enumerate() {

            if idx % 1000 == 0
            {
                let per_cent_remaining = ((idx as f32 / self.data.len() as f32) * 100.0) as i32;
                print!("\rSaving image {}%", per_cent_remaining);
                stdout.flush().unwrap();
            }

            let r = (255.999 * d[0]) as u8; 
            let g = (255.999 * d[1]) as u8; 
            let b = (255.999 * d[2]) as u8;

            img_payload += &format!("{0} {1} {2}\n", r, g, b);        
        }

        img.write(img_payload.as_bytes()).expect(&format!("writing of image payload {} failed", dest));
    }
}
