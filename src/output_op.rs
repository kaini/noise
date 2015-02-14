use noise::Noise;
use std::ops::Fn;

/// Applies an operation to the output of the source noise.
#[derive(Clone)]
pub struct OutputOp<In, Out, SrcOut, Src: Noise<In, SrcOut>, Op: Fn(SrcOut) -> Out> {
    // TODO Unboxed closures
    source: Src,
    op: Op,
}

impl<In, Out, SrcOut, Src: Noise<In, SrcOut>, Op: Fn(SrcOut) -> Out>
        OutputOp<In, Out, SrcOut, Src, Op> {

    pub fn new(source: Src, op: Op) -> OutputOp<In, Out, SrcOut, Src, Op> {
        OutputOp{
            source: source,
            op: op
        }
    }
}

impl<In, Out, SrcOut, Src: Noise<In, SrcOut>, Op: Fn(SrcOut) -> Out>
        Noise<In, Out>
        for OutputOp<In, Out, SrcOut, Src, Op> {

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
