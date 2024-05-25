use super::{ DoubleEndedIter, Iter, SizeHint };

pub struct Rev<I> {
	iter: I
}

impl<I: DoubleEndedIter> Rev<I> {
	pub(super) fn new(iter: I) -> Self {
		Self { iter }
	}

	pub fn into_inner(self) -> I {
		self.iter
	}
}

impl<I: DoubleEndedIter> Iter for Rev<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<I::Item> {
		self.iter.next_back()
	}

	fn size_hint(&self) -> SizeHint {
		self.iter.size_hint()
	}
}

impl<I: DoubleEndedIter> DoubleEndedIter for Rev<I> {
	fn next_back(&mut self) -> Option<I::Item> {
		self.iter.next()
	}
}
