use crate::prelude_std::*;

pub trait Input: Sized {
	type ConstSize<const N: usize>: Sized;

	fn len(&self) -> usize;

	#[inline]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn take_first(&self, i: usize) -> Option<(Self, Self)>;
	fn take_first_const<const N: usize>(&self) -> Option<(Self::ConstSize<N>, Self)>;
}

#[expect(clippy::len_without_is_empty, reason = "s h u t")]
pub trait Needle<I>
where
	I: Input
{
	fn len(&self) -> usize;
	fn input_starts_with(&self, input: &I) -> bool;
}
