use super::Iter;

pub trait IntoIter {
	type Item;
	type Iter: Iter<Item = Self::Item>;

	fn into_iter(self) -> Self::Iter;
}

impl<I: Iter> IntoIter for I {
	type Item = I::Item;
	type Iter = Self;

	#[inline]
	fn into_iter(self) -> Self {
		self
	}
}
