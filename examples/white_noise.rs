#![feature(path)]
extern crate image;
extern crate noise;
extern crate rand;

use image::{ImageBuffer, Rgb};
use noise::Noise;
use noise::blocks::new_white_noise;

fn main() {
    let noise = new_white_noise(rand::random(), 0.0, 255.999);
    let size = 128;

    let image = ImageBuffer::from_fn(size, size, Box::new(|x: u32, y: u32| {
        let v = noise.value((x * size + y) as i32) as u8;
        Rgb([v, v, v])
    }));
    match image.save(&Path::new("output.png")) {
        Err(e) => panic!("Could not write file! {}", e),
        Ok(..) => {},
    };
}

