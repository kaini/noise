use std::num::{Float, FloatMath};

pub trait Interpolator {
    fn interpolate(&self, a: f64, b: f64, percent: f64) -> f64;
}

pub struct LinearInterpolator;

impl Interpolator for LinearInterpolator {
    fn interpolate(&self, a: f64, b: f64, percent: f64) -> f64 {
        debug_assert!(0.0 <= percent && percent <= 1.0);
        a * (1.0 - percent) + b * percent
    }
}

pub struct PerlinInterpolator;

impl Interpolator for PerlinInterpolator {
    fn interpolate(&self, a: f64, b: f64, percent: f64) -> f64 {
        debug_assert!(0.0 <= percent && percent <= 1.0);
        // 3x^2 - 2x^3
        let x = percent * percent * (3.0 - 2.0 * percent);
        LinearInterpolator.interpolate(a, b, x)
    }
}

pub struct CosInterpolator;

impl Interpolator for CosInterpolator {
    fn interpolate(&self, a: f64, b: f64, percent: f64) -> f64 {
        debug_assert!(0.0 <= percent && percent <= 1.0);
        let x = (1.0 - (percent * Float::pi()).cos()) / 2.0;
        LinearInterpolator.interpolate(a, b, x)
    }
}

#[cfg(test)]
mod test {
    use super::{LinearInterpolator, PerlinInterpolator, CosInterpolator, Interpolator};
    use std::num::abs;

    #[test]
    fn interpolate_linear_test() {
        let a = 10.0;
        let b = 20.0;
        let result = LinearInterpolator.interpolate(a, b, 0.75);
        assert!(abs(result - 17.5) < 0.0001);
    }

    #[test]
    fn interpolate_perlin_test() {
        let a = 10.0;
        let b = 20.0;
        let result = PerlinInterpolator.interpolate(a, b, 0.75);
        assert!(abs(result - 18.4375) < 0.0001);
    }

    #[test]
    fn interpolate_cos_test() {
        let a = 10.0;
        let b = 20.0;
        let result = CosInterpolator.interpolate(a, b, 0.75);
        assert!(abs(result - 18.5355) < 0.0001);
    }
}
