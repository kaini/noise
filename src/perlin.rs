use noise::Noise;
use std::num::{Zero, Float};
use std::clone::Clone;

pub struct PerlinNoise<P: Clone, R: Float, N: Noise<P, R>> {
    octaves: Vec<N>,
}

impl<P: Clone, R: Float, N: Noise<P, R>> PerlinNoise<P, R, N> {
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
    use noise::Noise;
    use interpolate::LinearInterpolator;

    #[test]
    fn perlin_noise_test() {
        let octaves = vec![
            SmoothNoise1D::new(0, 1.0, 1.0, LinearInterpolator),
            SmoothNoise1D::new(0, 0.5, 2.0, LinearInterpolator),
        ];
        let noise = PerlinNoise::new(octaves);
        let range = &mut range(-200i, 201).map(|x| x as f64 / 10.0);
        for i in range {
            let value = noise.value(i);
            assert!(-1.5 <= value && value <= 1.5);
        }
    }
}