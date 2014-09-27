use std::num::Float;

pub trait Noise<P, R: Float> {
    fn value(&self, position: P) -> R;
}
