use interpolate::{Interpolator, sawtooth};
use noise::Noise;
use std::num::{NumCast, Float};

static X_PRIME: i32 = 1073741827;
static Y_PRIME: i32 = 53688275;

fn float_to_i32<F: Float>(f: F) -> i32 {
    NumCast::from(f).expect("got position that is not convertible to i32")
}

/// Takes discrete input noise to produce value-interpolated
/// output noise.
pub struct InterpolatedNoise<Src, I> {
    source: Src,
    interpolator: I,
}

impl<Src, I> InterpolatedNoise<Src, I> {
    /// Creates a new interpolated noise using the given source noise and the given
    /// interpolator.
    pub fn new(source: Src, interpolator: I) -> InterpolatedNoise<Src, I> {
        InterpolatedNoise{
            source: source,
            interpolator: interpolator
        }
    }
}

// TODO remove the i32 restriction to use any intregral type!
impl<InOut: Float, Src: Noise<i32, Out=InOut>, I: Interpolator<InOut>>
        Noise<InOut>
        for InterpolatedNoise<Src, I> {

    type Out = InOut;

    fn value(&self, position: InOut) -> InOut {
        let a = float_to_i32(position.floor());
        let b = a + 1;
        let p = sawtooth(position);
        self.interpolator.interpolate(
            self.source.value(a),
            self.source.value(b),
            p)
    }
}

/// Produces two-dimensional gradient-noise using discrete one-dimensional source noise.
///
/// See also <https://en.wikipedia.org/wiki/Perlin_noise> for the implementation.
/// The source noise is required to be between [-1; 1] or [0; 1] and should be
/// uniformly distributed.
// TODO remove the i32 and f64 restrictions!
pub struct InterpolatedNoise2D<Src: Noise<i32, Out=f64>, I: Interpolator<f64>> {
    source: Src,
    interpolator: I,
}

impl<Src: Noise<i32, Out=f64>, I: Interpolator<f64>>
        InterpolatedNoise2D<Src, I> {

    pub fn new(source: Src, interpolator: I) -> InterpolatedNoise2D<Src, I> {
        InterpolatedNoise2D{
            source: source,
            interpolator: interpolator
        }
    }

    fn gradient(&self, x: i32, y: i32) -> (f64, f64) {
        let mut x_index = x * X_PRIME + y * Y_PRIME;
        let mut y_index = x_index + 15268783;
        loop {
            let x = self.source.value(x_index);
            let y = self.source.value(y_index);
            let dist = x.powi(2) + y.powi(2);
            if dist <= 1.0 {
                let norm = dist.sqrt();
                return (x / norm, y / norm);
            }
            x_index *= 6684817;
            y_index *= 5684659;
        }
    }
}

impl<Src: Noise<i32, Out=f64>, I: Interpolator<f64>>
        Noise<(f64, f64)>
        for InterpolatedNoise2D<Src, I> {

    type Out = f64;

    fn value(&self, (pos_x, pos_y): (f64, f64)) -> f64 {

        fn dot(pos_x: f64, pos_y: f64, grad_x: i32, grad_y: i32, (gx, gy): (f64, f64)) -> f64 {
            let dx = pos_x - (grad_x as f64);
            let dy = pos_y - (grad_y as f64);
            dx * gx + dy * gy
        }

        let x0 = pos_x.floor() as i32;
        let x1 = x0 + 1;
        let y0 = pos_y.floor() as i32;
        let y1 = y0 + 1;

        let px = sawtooth(pos_x);
        let py = sawtooth(pos_y);

        // gradXY for X,Y <- {0,1}
        let grad00 = self.gradient(x0, y0);
        let grad01 = self.gradient(x0, y1);
        let grad10 = self.gradient(x1, y0);
        let grad11 = self.gradient(x1, y1);

        let n00 = dot(pos_x, pos_y, x0, y0, grad00);
        let n01 = dot(pos_x, pos_y, x0, y1, grad01);
        let n10 = dot(pos_x, pos_y, x1, y0, grad10);
        let n11 = dot(pos_x, pos_y, x1, y1, grad11);

        // interpolate both x directions
        let val1 = self.interpolator.interpolate(n00, n10, px);
        let val2 = self.interpolator.interpolate(n01, n11, px);

        // interpolate in y direction
        self.interpolator.interpolate(val1, val2, py)
    }
}

#[cfg(test)]
mod test {
    use super::{InterpolatedNoise, InterpolatedNoise2D};
    use noise::Noise;
    use default_noise::{NoOpNoise, DefaultI32Noise};
    use interpolate::LinearInterpolator;
    use output_op::OutputOp;
    use std::num::Float;

    #[test]
    fn interpolated_noise_test() {
        let noise = InterpolatedNoise::new(OutputOp::new(NoOpNoise, |i: i32| { i as f64 }), LinearInterpolator);
        for i in -100..100 {
            let f = i as f64 / 7.0;
            assert!((noise.value(f) - f).abs() < 0.001);
        }
    }

    #[test]
    fn interpolated_noise_2d_test() {
        let noise = InterpolatedNoise2D::new(DefaultI32Noise::new(0), LinearInterpolator);
        for i in -100..100 {
            for j in -100..100 {
                let f = i as f64 / 7.0;
                let g = j as f64 / 7.0;
                let value = noise.value((f, g));
                assert!(-1.0 <= value && value <= 1.0);
            }
        }
    }
}
