#![feature(path, core)]
extern crate image;
extern crate noise;
extern crate rand;

use image::{GenericImage, ImageBuffer, Rgb};
use noise::Noise;
use noise::blocks::new_perlin_noise_1d;

fn main() {
    let seed = rand::random();
    let amp = 60.0;
    let freq = 0.01;

    let noise = new_perlin_noise_1d(seed, amp, freq, 6);

    let img_height = (amp * 10.0 / 3.0 + 1.0) as u32;
    let mut image = ImageBuffer::from_pixel(512, img_height, Rgb([255, 255, 255]));

    for x in range(0u32, 512) {
        image.put_pixel(x, noise.value(x as f64) as u32 + img_height / 2, Rgb([0, 0, 0]));
    }

    match image.save(&Path::new("output.png")) {
        Err(e) => panic!("Could not write file! {}", e),
        Ok(..) => {},
    };  
}
