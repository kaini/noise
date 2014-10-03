//! This crate implements functions to generate one or multi dimensional smooth and Perlin noise.
//!
//! The `examples` folder contains several examples that show how to use this library.

pub use noise::{Noise, DefaultI32Noise};
pub use noise_1d::SmoothNoise1D;
pub use noise_2d::SmoothNoise2D;
pub use perlin::PerlinNoise;

pub mod interpolate;

mod noise;
mod noise_1d;
mod noise_2d;
mod perlin;
