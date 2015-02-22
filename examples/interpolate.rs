#![feature(old_path, core)]
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

    let mut image = ImageBuffer::from_pixel(512, amp as u32 * 3 + 30, Rgb([255, 255, 255]));

    for x in range(0, 512) {
        assert!(x < image.width());

        let a = (noise_a.value(x as f64) + 75.0) as u32;
        let b = (noise_b.value(x as f64) + 75.0) as u32 + 10;
        let c = (noise_c.value(x as f64) + 75.0) as u32 + 20;
        assert!(a < image.height());
        assert!(b < image.height());
        assert!(c < image.height());

        image.put_pixel(x, a, Rgb([255, 0, 0]));
        image.put_pixel(x, b, Rgb([0, 127, 0]));
        image.put_pixel(x, c, Rgb([0, 0, 255]));
    }

    match image.save(&Path::new("output.png")) {
        Err(e) => panic!("Could not write file! {}", e),
        Ok(..) => {},
    }; 
}
