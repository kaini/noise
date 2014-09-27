use interpolate::{Interpolator, position_to_percent};
use noise::{Noise, integer_noise};

static PRIME_X: i32 = 1956337;
static PRIME_Y: i32 = 8775607;
static PRIME_SEED: i32 = 967397;

#[deriving(PartialEq, PartialOrd, Show)]
pub struct SmoothNoise2D<I: Interpolator> {
    seed: i32,
    amplitude: f64,
    frequency: (f64, f64),
    interpolator: I,
}

impl<I: Interpolator> SmoothNoise2D<I> {
    pub fn new(seed: i32, amplitude: f64, frequency: (f64, f64), interpolator: I)
              -> SmoothNoise2D<I> {
        SmoothNoise2D{
            seed: seed,
            amplitude: amplitude,
            frequency: frequency,
            interpolator: interpolator,
        }
    }

    fn base_value(&self, position: (i32, i32)) -> f64 {
        let (x, y) = position;
        let n = x * PRIME_X + y * PRIME_Y + self.seed * PRIME_SEED;
        integer_noise(n) * self.amplitude
    }
}

impl<I: Interpolator> Noise<(f64, f64), f64> for SmoothNoise2D<I> {
    fn value(&self, position: (f64, f64)) -> f64 {
        let (fx, fy) = self.frequency;
        let (rawx, rawy) = position;
        let (x, y) = (rawx * fx, rawy * fy);
        let (basex, basey) = (x.floor() as i32, y.floor() as i32);

        // Interpolate x direction
        let x_percent = position_to_percent(x);
        let xval_a = self.interpolator.interpolate(self.base_value((basex, basey)),
                                                   self.base_value((basex + 1, basey)),
                                                   x_percent);
        let xval_b = self.interpolator.interpolate(self.base_value((basex, basey + 1)),
                                                   self.base_value((basex + 1, basey + 1)),
                                                   x_percent);

        // Interpolate y direction
        self.interpolator.interpolate(xval_a, xval_b, position_to_percent(y))
    }
}

#[cfg(test)]
mod test {
    use super::SmoothNoise2D;
    use interpolate::LinearInterpolator;
    use noise::Noise;

    #[test]
    fn smooth_noise_2d_test() {
        let noise = SmoothNoise2D::new(0, 1.0, (1.0, 1.0), LinearInterpolator);
        for x in range(-20i, 21) {
            for y in range(-20i, 21) {
                let pos = (x as f64 / 3.0, y as f64 / 3.0);
                let value = noise.value(pos);
                assert!(-1.0 <= value && value <= 1.0);
            }
        }
    }
}
