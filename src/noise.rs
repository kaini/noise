pub trait Noise<P, R> {
    fn value(&self, position: &P) -> R;
}
