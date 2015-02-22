use noise::Noise;

/// Applies an operation on the position parameter before the
/// position is passed to the source noise.
pub struct InputOp<Src, Op> {
	source: Src,
	op: Op,
}

impl<Src, Op> InputOp<Src, Op> {
	pub fn new(source: Src, op: Op) -> InputOp<Src, Op> {
		InputOp{
			source: source,
			op: op
		}
	}
}

impl<In, Out, OpOut, Src: Noise<OpOut, Out=Out>, Op: Fn(In) -> OpOut>
		Noise<In>
		for InputOp<Src, Op> {

	type Out = Out;

	fn value(&self, position: In) -> Out {
		self.source.value((self.op)(position))
	}
}

#[cfg(test)]
mod test {
	use super::InputOp;
	use default_noise::NoOpNoise;
	use noise::Noise;

	#[test]
	fn input_op_test() {
		let noise = InputOp::new(NoOpNoise, |p: i32| { p * 2 });
		for i in -10..10 {
			assert!(noise.value(i) == i * 2);
		}
	}
}
