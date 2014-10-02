use interpolate::{Interpolator, PerlinInterpolator, position_to_percent};
use noise::{Noise, DefaultI32Noise};
use std::rand;

static PRIME_POSITION: i32 = 999961;
static PRIME_SEED: i32 = 748361;

#[deriving(PartialEq, PartialOrd, Show)]
pub struct SmoothNoise1D<I: Interpolator, N: Noise<i32, f64>> {
    seed: i32,
    amplitude: f64,
    frequency: f64,
    interpolator: I,
    noise: N,
}

impl<I: Interpolator, N: Noise<i32, f64>> SmoothNoise1D<I, N> {
    pub fn new(seed: i32, amplitude: f64, frequency: f64, interpolator: I, noise: N)
              -> SmoothNoise1D<I, N> {
        SmoothNoise1D{
            seed: seed,
            amplitude: amplitude,
            frequency: frequency,
            interpolator: interpolator,
            noise: noise,
        }
    }

    fn base_value(&self, position: i32) -> f64 {
        self.noise.value(PRIME_POSITION * position + PRIME_SEED * self.seed) * self.amplitude
    }
}

impl SmoothNoise1D<PerlinInterpolator, DefaultI32Noise> {
    pub fn new_default(seed: i32, amplitude: f64, frequency: f64)
                      -> SmoothNoise1D<PerlinInterpolator, DefaultI32Noise> {
        SmoothNoise1D::new(seed, amplitude, frequency, PerlinInterpolator, DefaultI32Noise)
    }

    pub fn new_simple() -> SmoothNoise1D<PerlinInterpolator, DefaultI32Noise> {
        SmoothNoise1D::new_default(rand::random(), 1.0, 1.0)
    }
}

impl<I: Interpolator, N: Noise<i32, f64>> Noise<f64, f64> for SmoothNoise1D<I, N> {
    fn value(&self, position: f64) -> f64 {
        let position_scaled = position * self.frequency;
        let base_value_index = position_scaled.floor() as i32;
        let base_value_a = self.base_value(base_value_index);
        let base_value_b = self.base_value(base_value_index + 1);
        let percent = position_to_percent(position_scaled);
        let result = self.interpolator.interpolate(base_value_a, base_value_b, percent);
        //println!("pos {} idx {} a {} b {} frac {} result {}",
        //    position_scaled, base_value_index, base_value_a,
        //    base_value_b, percent, result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::SmoothNoise1D;
    use noise::{Noise, DefaultI32Noise};
    use interpolate::{LinearInterpolator, Interpolator};
    use std::num::{abs, signum};

    fn test_noise<I: Interpolator, N: Noise<i32, f64>, Iter: Iterator<f64>>(
            noise: &SmoothNoise1D<I, N>, min: f64, max: f64, max_delta: f64,
            monotone_steps: i32, iter: &mut Iter) {
        assert!(max_delta >= 0.0);

        let mut prev_value = None::<f64>;

        let mut prev_direction = None::<f64>;
        let mut prev_direction_switch = None::<i32>;

        for i in *iter {
            let value = noise.value(i);

            // Ensures that the values are inside the amplitude bounds
            assert!(min <= value && value <= max);

            // Tests the noise pattern for its frequency and first derivate
            match prev_value {
                Some(prev_value) => {
                    let delta = value - prev_value;

                    // Frequency
                    let direction = signum(delta);
                    match (prev_direction, prev_direction_switch) {
                        (Some(prev_direction), Some(pds)) => {
                            if prev_direction != direction {
                                assert!(pds >= monotone_steps);
                                prev_direction_switch = Some(1);
                            } else {
                                prev_direction_switch = Some(pds + 1);
                            }
                        }
                        (Some(prev_direction), None) => {
                            if prev_direction != direction {
                                prev_direction_switch = Some(1);
                            }
                        }
                        _ => {}
                    }

                    // Steepness (1st derivate)
                    assert!(abs(delta) <= max_delta);

                    prev_direction = Some(direction);
                }
                None => {}
            }

            prev_value = Some(value);
        }
    }

    #[test]
    fn smooth_noise_test() {
        let noise = &SmoothNoise1D::new(0, 1.0, 1.0, LinearInterpolator, DefaultI32Noise);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 13.0);
        test_noise(noise, -1.0, 1.0, 2.0 / 13.0, 13, range);
    }

    #[test]
    fn smooth_noise_low_frequency_test() {
        let noise = &SmoothNoise1D::new(0, 1.0, 1.0 / 100.0, LinearInterpolator, DefaultI32Noise);
        let range = &mut range(-2000i, 2001).map(|x| x as f64 / 7.0);
        test_noise(noise, -1.0, 1.0, 2.0 / 700.0, 700, range);
    }

    #[test]
    fn smooth_noise_high_frequency_test() {
        let noise = &SmoothNoise1D::new(0, 1.0, 100.0, LinearInterpolator, DefaultI32Noise);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 11.0);
        test_noise(noise, -1.0, 1.0, 2.0 / 0.11, 1, range);
    }

    #[test]
    fn smooth_noise_amplitude_test() {
        let noise = &SmoothNoise1D::new(0, 12.5, 1.0, LinearInterpolator, DefaultI32Noise);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 3.0);
        test_noise(noise, -12.5, 12.5, 25.0 / 3.0, 3, range);
    }

    #[test]
    fn new_defaults_test() {
        let noise = &SmoothNoise1D::new_default(0, 10.0, 1.0);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 7.0);
        test_noise(noise, -10.0, 10.0, 20.0 / 7.0 * 1.5, 7, range);
    }

    #[test]
    fn new_simple_test() {
        let noise = &SmoothNoise1D::new_simple();
        let range = &mut range(-200i, 201).map(|x| x as f64 / 10.0);
        test_noise(noise, -1.0, 1.0, 2.0 / 10.0 * 1.5, 10, range);
    }
}
