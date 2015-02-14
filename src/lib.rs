//! This crate implements functions to generate one or multi dimensional smooth and Perlin noise.
//!
//! The `examples` folder contains several examples that show how to use this library.

#![feature(core, test)]

extern crate test;

pub use noise::Noise;
pub use default_noise::{DefaultI32Noise, ConstantNoise, NoOpNoise, UnboxNoise};
pub use interpolated_noise::{InterpolatedNoise, InterpolatedNoise2D};
pub use output_op::OutputOp;
pub use input_op::InputOp;
pub use combined_noise::{CombinedNoise, CombinedNoise2};

pub mod interpolate;
pub mod blocks;

mod noise;
mod default_noise;
mod interpolated_noise;
mod output_op;
mod input_op;
mod combined_noise;
