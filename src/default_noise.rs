use noise::Noise;
use std::clone::Clone;

static PRIME_POSITION: i32 = 999961;
static PRIME_SEED: i32 = 748361;

/// Generator for pseudo-random integer noise.
///
/// This is implemented using `libnoise`'s integer noise function. See the [libnoise documentation](
/// http://libnoise.sourceforge.net/noisegen/#coherentnoise) for details.
pub struct DefaultI32Noise {
    seed: i32,
}

impl DefaultI32Noise {
    pub fn new(seed: i32) -> DefaultI32Noise {
        DefaultI32Noise{
            seed: (seed * PRIME_SEED) ^ (seed << 13),
        }
    }
}

impl Noise<i32> for DefaultI32Noise {
    type Out = f64;

    fn value(&self, position: i32) -> f64 {
        let a = position * PRIME_POSITION + self.seed;
        let b = (a << 13) ^ a;
        let c = (b * (b * b * 60493 + 19990303) + 1376312589) & 0x7fffffff;
        1.0 - (c as f64) / 1073741824.0
    }
}

/// Generator that always returns the same value.
pub struct ConstantNoise<Out> {
    value: Out,
}

impl<Out: Clone> ConstantNoise<Out> {
    pub fn new(value: Out) -> ConstantNoise<Out> {
        ConstantNoise{
            value: value
        }
    }
}

impl<In, Out: Clone> Noise<In> for ConstantNoise<Out> {
    type Out = Out;

    fn value(&self, _: In) -> Out {
        self.value.clone()
    }
}

/// Generator that always returns the position parameter unchanged.
pub struct NoOpNoise;

impl<In> Noise<In> for NoOpNoise {
    type Out = In;

    fn value(&self, position: In) -> In {
        position
    }
}

/// Wrapper that can be used to use `Box<Noise<...>>` as source to other
/// building blocks.
pub struct UnboxNoise<'a, In, Out> {
    source: Box<Noise<In, Out=Out> + 'a>
}

impl<'a, In, Out> UnboxNoise<'a, In, Out> {
    pub fn new(source: Box<Noise<In, Out=Out> + 'a>) -> UnboxNoise<'a, In, Out> {
        UnboxNoise{
            source: source
        }
    }
}

impl<'a, In, Out> Noise<In> for UnboxNoise<'a, In, Out> {
    type Out = Out;

    fn value(&self, position: In) -> Out {
        self.source.value(position)
    }
}


#[cfg(test)]
mod test {
    use super::{DefaultI32Noise, ConstantNoise, NoOpNoise};
    use std::iter::range_step_inclusive;
    use std::num::Int;
    use noise::Noise;

    #[test]
    fn integer_noise_test() {
        let noise = DefaultI32Noise::new(0);
        for n in range_step_inclusive(Int::min_value(), Int::max_value(), 1001) {
            let value = noise.value(n);
            assert!(-1.0 <= value && value <= 1.0);
        }
    }

    #[test]
    fn constant_noise_test() {
        let noise = ConstantNoise::new('x');
        for n in range_step_inclusive(Int::min_value(), Int::max_value(), 1001) {
            let value = noise.value(n);
            assert!(value == 'x');
        }
    }

    #[test]
    fn no_op_noise_test() {
        let noise = NoOpNoise;
        for n in range_step_inclusive(Int::min_value(), Int::max_value(), 1001) {
            let value = noise.value(n);
            assert!(value == n);
        }
    }
}
