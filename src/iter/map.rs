use super::{ Iter, SizeHint };

pub struct Map<I, F> {
	iter: I,
	f: F
}

impl<I, F, O> Map<I, F>
where
	I: Iter,
	F: FnMut(I::Item) -> O
{
	pub(super) fn new(iter: I, f: F) -> Self {
		Self { iter, f }
	}

	pub fn into_inner(self) -> (I, F) {
		let Self { iter, f } = self;
		(iter, f)
	}
}

impl<I, F, O> Iter for Map<I, F>
where
	I: Iter,
	F: FnMut(I::Item) -> O
{
	type Item = O;

	fn next(&mut self) -> Option<O> {
		self.iter.next().map(&mut self.f)
	}

	fn size_hint(&self) -> SizeHint {
		self.iter.size_hint()
	}
}
