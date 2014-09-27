pub use noise::{Noise, integer_noise};
pub use noise_1d::SmoothNoise1D;
pub use noise_2d::SmoothNoise2D;
pub use perlin::PerlinNoise;

pub mod interpolate;

mod noise;
mod noise_1d;
mod noise_2d;
mod perlin;
