#![feature(old_path)]
extern crate image;
extern crate noise;
extern crate rand;

use image::{ImageBuffer, Rgb};
use noise::Noise;
use noise::blocks::new_perlin_noise_2d;
use std::num::Float;

fn to_color(value: f64, factor: f64) -> u8 {
    let mut v = value.abs();
    if v > 255.0 {
        v = 255.0
    }
    (v * factor) as u8
}

fn main() {
    let amp = 255.0;
    let f = 0.01;

    let noise_r = new_perlin_noise_2d(rand::random(), amp, f, 6);
    let noise_g = new_perlin_noise_2d(rand::random(), amp, f, 6);
    let noise_b = new_perlin_noise_2d(rand::random(), amp, f, 6);

    let image = ImageBuffer::from_fn(128, 128, |x: u32, y: u32| {
        let p = (x as f64, y as f64);
        Rgb([
            to_color(noise_r.value(p), 1.0),
            to_color(noise_g.value(p), 1.0),
            to_color(noise_b.value(p), 1.0)
        ])
    });
    match image.save(&Path::new("output.png")) {
        Err(e) => panic!("Could not write file! {}", e),
        Ok(..) => {},
    };
}

