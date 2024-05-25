use super::{ Iter, Rev };

pub trait DoubleEndedIter: Iter {
	fn next_back(&mut self) -> Option<Self::Item>;

	fn rev(self) -> Rev<Self>
	where
		Self: Sized
	{
		Rev::new(self)
	}

	fn for_each_back<F>(mut self, mut f: F)
	where
		Self: Sized,
		F: FnMut(Self::Item)
	{
		self.rev().for_each(f)
	}
}

impl<I: DoubleEndedIter> DoubleEndedIter for &mut I {
	fn next_back(&mut self) -> Option<Self::Item> {
		(**self).next_back()
	}
}
