#![feature(old_path)]
extern crate image;
extern crate noise;
extern crate rand;

use image::{ImageBuffer, Rgb};
use noise::Noise;
use noise::blocks::new_noise_2d;
use std::num::Float;

fn main() {
    let amp = 76.5;
    let f = 0.05;
    let noise = new_noise_2d(rand::random(), amp, f);

    let size = 128;
    let image = ImageBuffer::from_fn(size, size, |x: u32, y: u32| {
        let v = (127.5 + noise.value((x as f64, y as f64))).round();
        Rgb([v as u8, v as u8, v as u8])
    });
    match image.save(&Path::new("output.png")) {
        Err(e) => panic!("Could not write file! {}", e),
        Ok(..) => {},
    };      
}

