extern crate image;
extern crate noise;

use image::{DynamicImage, ImageBuf, Rgb};
use noise::SmoothNoise2D;
use noise::PerlinNoise;
use noise::Noise;
use noise::interpolate::PerlinInterpolator;
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
    let f = 0.01;
    let octaves = vec![
        SmoothNoise2D::new(rand::random(), amp, (f, f), PerlinInterpolator),
        SmoothNoise2D::new(rand::random(), amp / 2.0, (f * 2.0, f * 2.0), PerlinInterpolator),
        SmoothNoise2D::new(rand::random(), amp / 4.0, (f * 4.0, f * 4.0), PerlinInterpolator),
        SmoothNoise2D::new(rand::random(), amp / 8.0, (f * 8.0, f * 8.0), PerlinInterpolator),
        SmoothNoise2D::new(rand::random(), amp / 16.0, (f * 16.0, f * 16.0), PerlinInterpolator),
        SmoothNoise2D::new(rand::random(), amp / 32.0, (f * 32.0, f * 32.0), PerlinInterpolator),
    ];
    let noise = PerlinNoise::new(octaves);

    let size = 512;
    let image = ImageBuf::from_fn(size, size, |x, y| {
        let v = (127.5 + noise.value((x as f64, y as f64))).round();
        Rgb(0, 0, v as u8)
    });
    write_png(&image::ImageRgb8(image));    
}

