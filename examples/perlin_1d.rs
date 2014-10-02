extern crate image;
extern crate noise;

use image::{DynamicImage, GenericImage, ImageBuf, Rgb};
use noise::{SmoothNoise1D, PerlinNoise, Noise};
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
    let amp = 60.0;
    let freq = 0.01;
    let octaves = vec![
        SmoothNoise1D::new_default(rand::random(), amp, freq),
        SmoothNoise1D::new_default(rand::random(), amp / 2.0, freq * 2.0),
        SmoothNoise1D::new_default(rand::random(), amp / 4.0, freq * 4.0),
        SmoothNoise1D::new_default(rand::random(), amp / 8.0, freq * 8.0),
        SmoothNoise1D::new_default(rand::random(), amp / 16.0, freq * 16.0),
        SmoothNoise1D::new_default(rand::random(), amp / 32.0, freq * 32.0),
    ];
    let noise = PerlinNoise::new(octaves);

    let img_height = (amp * 10.0 / 3.0 + 1.0) as u32;
    let mut image = ImageBuf::from_pixel(1000, img_height, Rgb(255, 255, 255));

    for x in range(0u32, 1000) {
        image.put_pixel(x, noise.value(x as f64) as u32 + img_height / 2, Rgb(0, 0, 0));
    }

    write_png(&image::ImageRgb8(image));    
}

