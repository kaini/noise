extern crate image;
extern crate noise;

use image::{DynamicImage, GenericImage, ImageBuf, Rgb};
use noise::SmoothNoise;
use noise::interpolate::{CosInterpolator, LinearInterpolator, PerlinInterpolator};
use std::io::File;
use std::path::Path;
use std::rand;

fn write_png(image: &DynamicImage) {
    let path = Path::new("output.png");
    let file = File::create(&path);
    match image.save(file, image::PNG) {
        Err(e) => fail!("Could not write file! {}", e),
        Ok(..) => {},
    };
}

fn main() {
    let seed = rand::random();
    let amp = 60.0;
    let freq = 0.02;

    let noise_a = SmoothNoise::new(seed, amp, freq, PerlinInterpolator);
    let noise_b = SmoothNoise::new(seed, amp, freq, CosInterpolator);
    let noise_c = SmoothNoise::new(seed, amp, freq, LinearInterpolator);

    let mut image = ImageBuf::from_pixel(1000, 150, Rgb(255, 255, 255));

    for x in range(0u32, 1000) {
        image.put_pixel(x, (noise_a.value(x as f64) + 75.0) as u32, Rgb(255, 0, 0));
        image.put_pixel(x, (noise_b.value(x as f64) + 75.0) as u32, Rgb(0, 127, 0));
        image.put_pixel(x, (noise_c.value(x as f64) + 75.0) as u32, Rgb(0, 0, 255));
    }

    write_png(&image::ImageRgb8(image));    
}

