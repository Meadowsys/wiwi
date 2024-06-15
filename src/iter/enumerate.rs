use super::{ Iter, SizeHintImpl, SizeHintMarker };

pub struct Enumerate<I> {
	iter: I,
	// pub(super) is for enumerate1
	pub(super) count: usize
}

impl<I> Enumerate<I>
where
	I: Iter
{
	pub(super) fn new(iter: I) -> Self {
		Self { iter, count: 0 }
	}
}

impl<I> Iter for Enumerate<I>
where
	I: Iter
{
	type Item = (I::Item, usize);

	fn next(&mut self) -> Option<(I::Item, usize)> {
		let next = self.iter.next()?;
		let next_i = self.count;

		self.count += 1;

		Some((next, next_i))
	}

	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		self.iter.size_hint().into()
	}
}
