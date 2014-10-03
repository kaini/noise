use noise::Noise;
use std::num::{Zero, Float};
use std::clone::Clone;

/// This is a noise generator that combines noise generators.
///
/// This generator creates noise that is commonly known as "Perlin Noise". Although this alogrithm
/// has not much to do with the original implementation of Perlin Noise it yields similar results.
///
/// The idea is really simple: Multiple smooth noises are added together.
///
/// In most common case Perlin noise consists of a smooth noise with a low frequency and a high
/// amplitude and 3 to 6 (depending on the amount of processing power at disposal and required
/// resolution) smooth noises with half the frequency and half the amplitude of the noise before.
/// (These single noise generators are commonly called octaves.)
///
/// The highest possible value is the sum of the highest possible values for all noise functions.
/// (Note: If you half the amplitude each step, the highest possible value is `1 + (2 / 3)` times
/// the base frequency.)
///
/// # Example
///
/// This code generates typical Perlin-like noise with a base frequency of 0.02, which means
/// 50 units.
///
/// ```rust
/// use noise::{SmoothNoise2D, PerlinNoise};
/// use std::rand::random;
///
/// let amp = 76.5;
/// let f = 0.02;
/// let octaves = vec![
///     SmoothNoise2D::new_default(random(), amp, (f, f)),
///     SmoothNoise2D::new_default(random(), amp / 2.0, (f * 2.0, f * 2.0)),
///     SmoothNoise2D::new_default(random(), amp / 4.0, (f * 4.0, f * 4.0)),
///     SmoothNoise2D::new_default(random(), amp / 8.0, (f * 8.0, f * 8.0)),
///     SmoothNoise2D::new_default(random(), amp / 16.0, (f * 16.0, f * 16.0)),
///     SmoothNoise2D::new_default(random(), amp / 32.0, (f * 32.0, f * 32.0)),
/// ];
/// let noise = PerlinNoise::new(octaves);
/// ```
///
/// You can find more exapmles in the `examples` directory.
pub struct PerlinNoise<P: Clone, R: Float, N: Noise<P, R>> {
    octaves: Vec<N>,
}

impl<P: Clone, R: Float, N: Noise<P, R>> PerlinNoise<P, R, N> {
    /// Creates a Perlin noise generator using the given noise generators.
    pub fn new(octaves: Vec<N>) -> PerlinNoise<P, R, N> {
        PerlinNoise{octaves: octaves}
    }
}

impl<P: Clone, R: Float, N: Noise<P, R>> Noise<P, R> for PerlinNoise<P, R, N> {
    fn value(&self, position: P) -> R {
        let mut result: R = Zero::zero();
        for noise in self.octaves.iter() {
            result = result + noise.value(position.clone());
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::PerlinNoise;
    use noise_1d::SmoothNoise1D;
    use noise::{Noise, DefaultI32Noise};
    use interpolate::LinearInterpolator;

    #[test]
    fn perlin_noise_test() {
        let octaves = vec![
            SmoothNoise1D::new(0, 1.0, 1.0, LinearInterpolator, DefaultI32Noise),
            SmoothNoise1D::new(0, 0.5, 2.0, LinearInterpolator, DefaultI32Noise),
        ];
        let noise = PerlinNoise::new(octaves);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 10.0);
        for i in range {
            let value = noise.value(i);
            assert!(-1.5 <= value && value <= 1.5);
        }
    }
}