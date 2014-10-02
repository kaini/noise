use std::num::Float;

pub trait Noise<P, R: Float> {
    fn value(&self, position: P) -> R;
}

pub struct DefaultI32Noise;

impl Noise<i32, f64> for DefaultI32Noise {
    /// http://libnoise.sourceforge.net/noisegen/#coherentnoise
    fn value(&self, a: i32) -> f64 {
        let b = (a >> 13) ^ a;
        let c = (b * (b * b * 60493 + 19990303) + 1376312589) & 0x7fffffff;
        let d = c as f64;
        1.0 - d / 1073741824.0
    }
}

#[cfg(test)]
mod test {
    use super::{DefaultI32Noise, Noise};
    use std::num::Bounded;
    use std::iter::range_step_inclusive;

    #[test]
    fn integer_noise_test() {
        for n in range_step_inclusive(Bounded::min_value(), Bounded::max_value(), 1001) {
            let value = DefaultI32Noise.value(n);
            assert!(-1.0 <= value && value <= 1.0);
        }
    }
}
