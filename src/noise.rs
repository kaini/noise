
/// A common interface for all structs that can generate noise.
pub trait Noise<P, R> {
    /// Calculates the value of the noise function at the given position.
    ///
    /// The noise for a position never changes.
    fn value(&self, position: P) -> R;
}

