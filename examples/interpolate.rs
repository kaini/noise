#![feature(path, core)]
extern crate image;
extern crate noise;
extern crate rand;

use image::{GenericImage, ImageBuffer, Rgb};
use noise::Noise;
use noise::blocks::new_noise_1d_int;
use noise::interpolate::{CosInterpolator, LinearInterpolator, PerlinInterpolator};
use std::old_path::Path;

fn main() {
    let seed = rand::random();
    let amp = 60.0;
    let freq = 0.02;

    let noise_a = new_noise_1d_int(seed, amp, freq, CosInterpolator);
    let noise_b = new_noise_1d_int(seed, amp, freq, PerlinInterpolator);
    let noise_c = new_noise_1d_int(seed, amp, freq, LinearInterpolator);

    let mut image = ImageBuffer::from_pixel(512, amp as u32 * 2 + 30, Rgb([255, 255, 255]));

    for x in range(0u32, 512) {
        image.put_pixel(x, (noise_a.value(x as f64) + 75.0) as u32, Rgb([255, 0, 0]));
        image.put_pixel(x, (noise_b.value(x as f64) + 75.0) as u32 + 10, Rgb([0, 127, 0]));
        image.put_pixel(x, (noise_c.value(x as f64) + 75.0) as u32 + 20, Rgb([0, 0, 255]));
    }

    match image.save(&Path::new("output.png")) {
        Err(e) => panic!("Could not write file! {}", e),
        Ok(..) => {},
    }; 
}
