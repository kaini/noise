//! Interpolating between two numbers.
//!
//! Interpolation is required to generate smooth noise.

use std::num::Float;
use std::f64::consts::PI;

/// Interface to interpolate beweeen two numbers.
pub trait Interpolator<F: Float> {
    /// Interpolates between the two numbers `a` and `b`. `percent` is a float between 0 and 1 (both
    /// borders inclusive).
    ///
    /// There are some laws implementors of this function have to obey:
    ///
    /// * The result of `interpolate(a, b, p)` never changes.
    /// * `interpolate(a, b, 1.0) == interpolate(b, c, 0.0)`
    /// * `interpolate(a, b, p) <= interpolate(a, b, q)` for all `p < q`
    ///
    /// The following law is sensible but not stricly required:
    ///
    /// * `interpolate(a, b, p) == a + b - interpolate(a, b, 1 - p)`: This means that the function
    /// has to be mirrored around the center point of the interpolated area. Literally all sane
    /// interpolation functions obey this law. This also implies that
    /// `interpolate(a, b, 0.5) == (a + b) / 2`.
    fn interpolate(&self, a: F, b: F, percent: F) -> F;
}

/// Interpolator that interoplates linear between `a` and `b`.
///
/// This means that `interpolate(a, b, p) == a * (1 - p) + b * p`. This [Wolfram Alpha plot](
/// http://www.wolframalpha.com/input/?i=plot+%28x%29+for+x+%3D+0+to+1) shows how this interpolation
/// function looks like.
pub struct LinearInterpolator;

impl Interpolator<f64> for LinearInterpolator {
    fn interpolate(&self, a: f64, b: f64, percent: f64) -> f64 {
        debug_assert!(0.0 <= percent && percent <= 1.0);
        a * (1.0 - percent) + b * percent
    }
}

/// Interpolates very similar to a cosine interpolation. This is the *recommended* interpolator.
///
/// This is the interpolation function used by Perlin in [the original Perlin Noise implementation](
/// http://mrl.nyu.edu/~perlin/doc/oscar.html). See [Wolfram Alpha](
/// http://www.wolframalpha.com/input/?i=plot+3x^2-2x^3+for+x+%3D+0+to+1) to get an idea of how this
/// looks like.
pub struct PerlinInterpolator;

impl Interpolator<f64> for PerlinInterpolator {
    fn interpolate(&self, a: f64, b: f64, percent: f64) -> f64 {
        debug_assert!(0.0 <= percent && percent <= 1.0);
        // 3x^2 - 2x^3
        let x = percent * percent * (3.0 - 2.0 * percent);
        LinearInterpolator.interpolate(a, b, x)
    }
}

/// Interpolates by using the cosine slope.
///
/// This is slightly better than Perlin Interpolation but much more expensive. See
/// [Wolfram Alpha](
/// http://www.wolframalpha.com/input/?i=plot+%281-cos%28pi*x%29%29%2F2+for+x+%3D+0+to+1) to get
/// an idea of how this looks like.
pub struct CosInterpolator;

impl Interpolator<f64> for CosInterpolator {
    fn interpolate(&self, a: f64, b: f64, percent: f64) -> f64 {
        debug_assert!(0.0 <= percent && percent <= 1.0);
        let x = (1.0 - (percent * PI).cos()) / 2.0;
        LinearInterpolator.interpolate(a, b, x)
    }
}

/// Sawtooth function in the range zero (inclusive) to one (exclusive) and a frequency of one.
pub fn sawtooth<F: Float>(x: F) -> F {
    x - x.floor()
}

#[cfg(test)]
mod test {
    use super::{LinearInterpolator, PerlinInterpolator, CosInterpolator, Interpolator, sawtooth};
    use std::num::Float;

    #[test]
    fn interpolate_linear_test() {
        let a = 10.0;
        let b = 20.0;
        let result = LinearInterpolator.interpolate(a, b, 0.75);
        assert!((result - 17.5).abs() < 0.0001);
    }

    #[test]
    fn interpolate_perlin_test() {
        let a = 10.0;
        let b = 20.0;
        let result = PerlinInterpolator.interpolate(a, b, 0.75);
        assert!((result - 18.4375).abs() < 0.0001);
    }

    #[test]
    fn interpolate_cos_test() {
        let a = 10.0;
        let b = 20.0;
        let result = CosInterpolator.interpolate(a, b, 0.75);
        assert!((result - 18.5355).abs() < 0.0001);
    }

    #[test]
    fn sawtooth_test() {
        assert!(sawtooth(0.0) == 0.0);

        assert!(sawtooth(1.0) == 0.0);
        assert!(sawtooth(-1.0) == 0.0);

        assert!(sawtooth(1.25) == 0.25);
        assert!(sawtooth(-1.25) == 0.75);
    }
}
