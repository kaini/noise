extern crate image;
extern crate noise;

use image::{DynamicImage, ImageBuf, Rgb};
use noise::SmoothNoise2D;
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
    let noise = SmoothNoise2D::new(rand::random(), 127.5, (0.02, 0.02), PerlinInterpolator);
    let size = 512;
    let image = ImageBuf::from_fn(size, size, |x, y| {
        let v = (127.5 + noise.value((x as f64, y as f64))).round();
        Rgb(0, 0, v as u8)
    });
    write_png(&image::ImageRgb8(image));    
}

