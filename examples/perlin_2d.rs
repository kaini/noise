extern crate image;
extern crate noise;

use image::{DynamicImage, ImageBuf, Rgb};
use noise::{SmoothNoise2D, PerlinNoise, Noise};
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
    let amp = 76.5;
    let f = 0.02;
    let octaves = vec![
        SmoothNoise2D::new_default(rand::random(), amp, (f, f)),
        SmoothNoise2D::new_default(rand::random(), amp / 2.0, (f * 2.0, f * 2.0)),
        SmoothNoise2D::new_default(rand::random(), amp / 4.0, (f * 4.0, f * 4.0)),
        SmoothNoise2D::new_default(rand::random(), amp / 8.0, (f * 8.0, f * 8.0)),
        SmoothNoise2D::new_default(rand::random(), amp / 16.0, (f * 16.0, f * 16.0)),
        SmoothNoise2D::new_default(rand::random(), amp / 32.0, (f * 32.0, f * 32.0)),
    ];
    let noise = PerlinNoise::new(octaves);

    let size = 512;
    let image = ImageBuf::from_fn(size, size, |x, y| {
        let v = (127.5 + noise.value((x as f64, y as f64))).round();
        Rgb(0, 0, v as u8)
    });
    write_png(&image::ImageRgb8(image));    
}

