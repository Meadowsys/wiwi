use super::{ Enumerate, Iter, SizeHintImpl, SizeHintMarker };

pub struct Enumerate1<I> {
	// just sets `inner.count` to 1 and then piggy backs off of it
	inner: Enumerate<I>
}

impl<I> Enumerate1<I>
where
	I: Iter
{
	pub(super) fn new(iter: I) -> Self {
		let mut inner = Enumerate::new(iter);
		inner.count = 1;
		Self { inner }
	}
}

impl<I> Iter for Enumerate1<I>
where
	I: Iter
{
	type Item = (I::Item, usize);

	fn next(&mut self) -> Option<(I::Item, usize)> {
		self.inner.next()
	}

	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		self.inner.size_hint().into()
	}
}
