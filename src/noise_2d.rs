use interpolate::{Interpolator, sawtooth, PerlinInterpolator};
use noise::{Noise, DefaultI32Noise};
use std::rand;

static PRIME_X: i32 = 1956337;
static PRIME_Y: i32 = 8775607;
static PRIME_SEED: i32 = 967397;

/// Generator for two dimensional smooth noise.
pub struct SmoothNoise2D<I: Interpolator, N: Noise<i32, f64>> {
    seed: i32,
    amplitude: f64,
    frequency: (f64, f64),
    interpolator: I,
    noise: N,
}

impl<I: Interpolator, N: Noise<i32, f64>> SmoothNoise2D<I, N> {
    /// Creates a noise generator for two dimensional noise.
    ///
    /// For a detailed description of the parameters see `SmoothNoise1D`.
    pub fn new(seed: i32, amplitude: f64, frequency: (f64, f64), interpolator: I, noise: N)
              -> SmoothNoise2D<I, N> {
        SmoothNoise2D{
            seed: seed,
            amplitude: amplitude,
            frequency: frequency,
            interpolator: interpolator,
            noise: noise,
        }
    }

    fn base_value(&self, position: (i32, i32)) -> f64 {
        let (x, y) = position;
        let n = x * PRIME_X + y * PRIME_Y + self.seed * PRIME_SEED;
        self.noise.value(n) * self.amplitude
    }
}

impl SmoothNoise2D<PerlinInterpolator, DefaultI32Noise> {
    /// Creates a generator for two dimensional smooth noise using sensible defaults.
    ///
    /// For interpolation the `PerlinInterpolator` will be used and as base noise `DefaultI32Noise`
    /// will be used.
    pub fn new_default(seed: i32, amplitude: f64, frequency: (f64, f64))
                      -> SmoothNoise2D<PerlinInterpolator, DefaultI32Noise> {
        SmoothNoise2D::new(seed, amplitude, frequency, PerlinInterpolator, DefaultI32Noise)
    }

    /// Creates a randomly seeded generator for two dimensional smoooth noise.
    ///
    /// This generator will have an amplitude and frequency of one and use the `PerlinIntepolator`
    /// as interpolator and `DefaultI32Noise` as base noise. The seed is chosen by
    /// `std::rand::random()`.
    pub fn new_simple() -> SmoothNoise2D<PerlinInterpolator, DefaultI32Noise> {
        SmoothNoise2D::new_default(rand::random(), 1.0, (1.0, 1.0))
    }
}

impl<I: Interpolator, N: Noise<i32, f64>> Noise<(f64, f64), f64> for SmoothNoise2D<I, N> {
    fn value(&self, position: (f64, f64)) -> f64 {
        let (fx, fy) = self.frequency;
        let (rawx, rawy) = position;
        let (x, y) = (rawx * fx, rawy * fy);
        let (basex, basey) = (x.floor() as i32, y.floor() as i32);

        // Interpolate x direction
        let x_percent = sawtooth(x);
        let xval_a = self.interpolator.interpolate(self.base_value((basex, basey)),
                                                   self.base_value((basex + 1, basey)),
                                                   x_percent);
        let xval_b = self.interpolator.interpolate(self.base_value((basex, basey + 1)),
                                                   self.base_value((basex + 1, basey + 1)),
                                                   x_percent);

        // Interpolate y direction
        self.interpolator.interpolate(xval_a, xval_b, sawtooth(y))
    }
}

#[cfg(test)]
mod test {
    use super::SmoothNoise2D;
    use interpolate::LinearInterpolator;
    use noise::{Noise, DefaultI32Noise};

    #[test]
    fn smooth_noise_2d_test() {
        let noise = SmoothNoise2D::new(0, 1.0, (1.0, 1.0), LinearInterpolator, DefaultI32Noise);
        for x in range(-20i, 21) {
            for y in range(-20i, 21) {
                let pos = (x as f64 / 3.0, y as f64 / 3.0);
                let value = noise.value(pos);
                assert!(-1.0 <= value && value <= 1.0);
            }
        }
    }

    #[test]
    fn new_default_test() {
        let noise = SmoothNoise2D::new_default(0, 1.0, (1.0, 1.0));
        for x in range(-20i, 21) {
            for y in range(-20i, 21) {
                let pos = (x as f64 / 3.0, y as f64 / 3.0);
                let value = noise.value(pos);
                assert!(-1.0 <= value && value <= 1.0);
            }
        }
    }

    #[test]
    fn new_simple_test() {
        let noise = SmoothNoise2D::new_simple();
        for x in range(-20i, 21) {
            for y in range(-20i, 21) {
                let pos = (x as f64 / 3.0, y as f64 / 3.0);
                let value = noise.value(pos);
                assert!(-1.0 <= value && value <= 1.0);
            }
        }
    }
}
