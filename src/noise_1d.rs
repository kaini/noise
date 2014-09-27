use interpolate::Interpolator;
use std::num::Float;
use std::rand::{SeedableRng, IsaacRng, Closed01, Rng};

#[deriving(PartialEq, PartialOrd, Show)]
pub struct SmoothNoise1D<I: Interpolator> {
    seed: u32,
    amplitude: f64,
    frequency: f64,
    interpolator: I,
}

impl<I: Interpolator> SmoothNoise1D<I> {
    pub fn new(seed: u32, amplitude: f64, frequency: f64, interpolator: I) -> SmoothNoise1D<I> {
        SmoothNoise1D{
            seed: seed,
            amplitude: amplitude,
            frequency: frequency,
            interpolator: interpolator,
        }
    }

    pub fn value(&self, position: f64) -> f64 {
        let position_scaled = position * self.frequency;
        let base_value_index = position_scaled.floor() as i32;
        let base_value_a = self.base_value(base_value_index);
        let base_value_b = self.base_value(base_value_index + 1);
        let position_between_ab = 
            if position_scaled < 0.0 {
                let fract = position_scaled.fract();
                if fract == 0.0 { 0.0 } else { 1.0 + position_scaled.fract() }
            } else {
                position_scaled.fract()
            };
        let result = self.interpolator.interpolate(base_value_a, base_value_b, position_between_ab);
        //println!("pos {} idx {} a {} b {} frac {} result {}",
        //    position_scaled, base_value_index, base_value_a,
        //    base_value_b, position_between_ab, result);
        result
    }

    fn base_value(&self, position: i32) -> f64 {
        let mut rng: IsaacRng = SeedableRng::from_seed([self.seed, position as u32].as_slice());
        let Closed01(value) = rng.gen::<Closed01<f64>>();
        (value - 0.5) * 2.0 * self.amplitude
    }
}

pub struct PerlinNoise1D<I: Interpolator> {
    octaves: Vec<SmoothNoise1D<I>>,
}

impl<I: Interpolator> PerlinNoise1D<I> {
    pub fn new(octaves: Vec<SmoothNoise1D<I>>) -> PerlinNoise1D<I> {
        PerlinNoise1D{octaves: octaves}
    }

    pub fn value(&self, position: f64) -> f64 {
        let mut result = 0.0;
        for noise in self.octaves.iter() {
            result += noise.value(position)
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::{SmoothNoise1D, PerlinNoise1D};
    use interpolate::{LinearInterpolator, Interpolator};
    use std::num::{abs, signum};

    fn test_noise<I: Interpolator, Iter: Iterator<f64>>(
            noise: &SmoothNoise1D<I>, min: f64, max: f64, max_delta: f64,
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
        let noise = &SmoothNoise1D::new(0, 1.0, 1.0, LinearInterpolator);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 13.0);
        test_noise(noise, -1.0, 1.0, 2.0 / 13.0, 13, range);
    }

    #[test]
    fn smooth_noise_low_frequency_test() {
        let noise = &SmoothNoise1D::new(0, 1.0, 1.0 / 100.0, LinearInterpolator);
        let range = &mut range(-2000i, 2001).map(|x| x as f64 / 7.0);
        test_noise(noise, -1.0, 1.0, 2.0 / 700.0, 700, range);
    }

    #[test]
    fn smooth_noise_high_frequency_test() {
        let noise = &SmoothNoise1D::new(0, 1.0, 100.0, LinearInterpolator);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 11.0);
        test_noise(noise, -1.0, 1.0, 2.0 / 0.11, 1, range);
    }

    #[test]
    fn smooth_noise_amplitude_test() {
        let noise = &SmoothNoise1D::new(0, 12.5, 1.0, LinearInterpolator);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 3.0);
        test_noise(noise, -12.5, 12.5, 25.0 / 3.0, 3, range);
    }

    #[test]
    // TODO abstract noise generator
    fn perlin_noise_test() {
        let octaves = vec![
            SmoothNoise1D::new(0, 1.0, 1.0, LinearInterpolator),
            SmoothNoise1D::new(0, 0.5, 2.0, LinearInterpolator),
        ];
        let noise = PerlinNoise1D::new(octaves);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 10.0);
        for i in range {
            let value = noise.value(i);
            assert!(-1.5 <= value && value <= 1.5);
        }
    }
}
