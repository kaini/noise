use noise::Noise;
use std::ops::Fn;

/// Applies an operation to the output of the source noise.
pub struct OutputOp<Src, Op> {
    // TODO Unboxed closures
    source: Src,
    op: Op,
}

impl<Src, Op> OutputOp<Src, Op> {
    pub fn new(source: Src, op: Op) -> OutputOp<Src, Op> {
        OutputOp{
            source: source,
            op: op
        }
    }
}

impl<In, Mid, Out, Src: Noise<In, Out=Mid>, Op: Fn(Mid) -> Out>
        Noise<In>
        for OutputOp<Src, Op> {

    type Out = Out;

    fn value(&self, position: In) -> Out {
        let value = self.source.value(position);
        (self.op)(value)
    }
}

#[cfg(test)]
mod test {
    use super::OutputOp;
    use default_noise::NoOpNoise;
    use noise::Noise;

    #[test]
    fn output_op_test() {
        let noise = OutputOp::new(NoOpNoise, |i: i32| { i * 2 });
        for i in -10..10 {
            assert!(noise.value(i) == i * 2);
        }
    }
}
