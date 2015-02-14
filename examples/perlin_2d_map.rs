#![feature(path)]
extern crate image;
extern crate noise;
extern crate rand;

use image::{ImageBuffer, Rgb};
use noise::{Noise, OutputOp, UnboxNoise};
use noise::blocks::new_perlin_noise_2d;

fn map(value: f64, from_a: f64, from_b: f64, to_a: u8, to_b: u8) -> u8 {
    assert!(from_a <= value && value <= from_b);
    let from_width = from_b - from_a;
    let norm = (value - from_a) / from_width;
    let to_width = to_b - to_a;
    (norm * to_width as f64) as u8 + to_a
}

fn main() {
    let amp = 1.0;
    let f = 0.01;

    let noise = OutputOp::new(
        UnboxNoise::new(new_perlin_noise_2d(rand::random(), amp, f, 6)),
        |h: f64| {
            if h < 0.0 {
                (0, 0, map(h, -1.0, 0.0, 100, 200))
            } else if h < 0.05 {
                (255, 235, map(h, 0.0, 0.1, 180, 205))
            } else if h < 0.45 {
                (0, 128 - map(h, 0.05, 0.45, 0, 48), 0)
            } else if h < 0.65 {
                let g = map(h, 0.45, 0.65, 100, 200);
                (g, g, g)
            } else {
                let g = map(h, 0.65, 1.0, 230, 255);
                (g, g, g)
            }
        }
    );

    let size = 128;
    let image = ImageBuffer::from_fn(size, size, Box::new(|x: u32, y: u32| {
        let (r, g, b) = noise.value((x as f64, y as f64));
        Rgb([r, g, b])
    }));
    match image.save(&Path::new("output.png")) {
        Err(e) => panic!("Could not write file! {}", e),
        Ok(..) => {},
    };
}

