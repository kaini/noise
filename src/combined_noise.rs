use noise::Noise;

/// Creates noise by combining multiple source noises.
///
/// This noise uses trait objects to manage the multiple source noises.
pub struct CombinedNoise<'a, In: Clone, Out, Combine: Fn(Out, Out) -> Out> {
	sources: Vec<Box<Noise<In, Out> + 'a>>,
	combine: Combine,
}

impl<'a, In: Clone, Out, Combine: Fn(Out, Out) -> Out>
		CombinedNoise<'a, In, Out, Combine> {

	pub fn new(sources: Vec<Box<Noise<In, Out> + 'a>>, combine: Combine)
			-> CombinedNoise<'a, In, Out, Combine> {
		assert!(!sources.is_empty());
		CombinedNoise{
			sources: sources,
			combine: combine
		}
	}
}

impl<'a, In: Clone, Out, Combine: Fn(Out, Out) -> Out>
		Noise<In, Out>
		for CombinedNoise<'a, In, Out, Combine> {

	fn value(&self, position: In) -> Out {
		let mut value = self.sources[0].value(position.clone());
		for i in 1..self.sources.len() {
			value = (self.combine)(value, self.sources[i].value(position.clone()))
		}
		value
	}
}

/// Creates a noise that combines two source noises.
///
/// This generator does not use trait objects, but static dispatch.
pub struct CombinedNoise2<In: Clone, Out, Src1Out, Src2Out, Src1: Noise<In, Src1Out>, Src2: Noise<In, Src2Out>, Combine: Fn(Src1Out, Src2Out) -> Out> {
	source1: Src1,
	source2: Src2,
	combine: Combine,
}

impl<In: Clone, Out, Src1Out, Src2Out, Src1: Noise<In, Src1Out>, Src2: Noise<In, Src2Out>, Combine: Fn(Src1Out, Src2Out) -> Out>
		CombinedNoise2<In, Out, Src1Out, Src2Out, Src1, Src2, Combine> {

	pub fn new(source1: Src1, source2: Src2, combine: Combine)
			-> CombinedNoise2<In, Out, Src1Out, Src2Out, Src1, Src2, Combine> {
		CombinedNoise2{
			source1: source1,
			source2: source2,
			combine: combine
		}
	}
}

impl<In: Clone, Out, Src1Out, Src2Out, Src1: Noise<In, Src1Out>, Src2: Noise<In, Src2Out>, Combine: Fn(Src1Out, Src2Out) -> Out>
		Noise<In, Out>
		for CombinedNoise2<In, Out, Src1Out, Src2Out, Src1, Src2, Combine> {

	fn value(&self, position: In) -> Out {
		let value1 = self.source1.value(position.clone());
		let value2 = self.source2.value(position);
		(self.combine)(value1, value2)
	}
}

#[cfg(test)]
mod test {
	use super::{CombinedNoise, CombinedNoise2};
	use default_noise::{NoOpNoise, ConstantNoise};
	use noise::Noise;

	#[test]
	fn combined_noise_test() {
		let noise = CombinedNoise::new(
			vec![Box::new(ConstantNoise::new(5)), Box::new(NoOpNoise), Box::new(ConstantNoise::new(-1))],
			|a: i32, b: i32| { a + b }
		);
		for i in -10..10 {
			assert!(noise.value(i) == i + 4);
		}
	}

	#[test]
	fn combined_noise2_test() {
		let noise = CombinedNoise2::new(NoOpNoise, NoOpNoise, |a: i32, b: i32| { a * b });
		for i in -10..10 {
			assert!(noise.value(i) == i * i);
		}
	}
}
