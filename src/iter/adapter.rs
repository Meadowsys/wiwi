use super::{ IntoIter, Iter };

/// An adapter that wraps either a std [`Iterator`] or a wiwi [`Iter`], and then
/// acts as the other. Basically, with every layer you layer it, it flip flops
/// back and forth lol
///
/// If you want to convert one type of iter to the other, you're probably looking
/// for [`into_wiwi_iter`](IntoWiwiIter::into_wiwi_iter)
/// or [`into_std_iterator`](IntoStdIterator::into_std_iterator).
///
/// If you want to borrow an iter as the other, you're probably looking
/// for [`as_wiwi_iter`](AsWiwiIter::as_wiwi_iter)
/// or [`as_std_iterator`](AsStdIterator::as_std_iterator)
#[repr(transparent)]
pub struct IterAdapter<I> {
	inner: I
}

impl<I: Iter> Iterator for IterAdapter<I> {
	type Item = I::Item;
	fn next(&mut self) -> Option<I::Item> {
		self.inner.next()
	}
}

impl<I: Iterator> Iter for IterAdapter<I> {
	type Item = I::Item;
	fn next(&mut self) -> Option<I::Item> {
		self.inner.next()
	}
}

/// Allows any std iterator to be borrowed as a wiwi iter. There is a
/// blanket implementation provided, so it is available for all std iterators.
pub trait AsWiwiIter<'h> {
	type Iter: Iter + 'h;
	fn as_wiwi_iter(&'h mut self) -> Self::Iter;
}

impl<'h, I: Iterator + 'h> AsWiwiIter<'h> for I {
	type Iter = IterAdapter<&'h mut I>;
	fn as_wiwi_iter(&'h mut self) -> IterAdapter<&'h mut Self> {
		IterAdapter { inner: self }
	}
}

/// Allows any wiwi iter to be borrowed as an std iterator. There is a
/// blanket implementation provided, so it is available for all wiwi iters.
pub trait AsStdIterator<'h> {
	type Iterator: Iterator + 'h;
	fn as_std_iterator(&'h mut self) -> Self::Iterator;
}

impl<'h, I: Iter + 'h> AsStdIterator<'h> for I {
	type Iterator = IterAdapter<&'h mut I>;
	fn as_std_iterator(&'h mut self) -> IterAdapter<&'h mut Self> {
		IterAdapter { inner: self }
	}
}

/// Allows any std iterator, or anything capable of converting to an std iterator
/// (ie. implements [`IntoIterator`]), to convert to a wiwi iter. There is a blanket
/// implementation provided, so it is available for all std iterators.
pub trait IntoWiwiIter {
	type Iter: Iter;
	fn into_wiwi_iter(self) -> Self::Iter;
}

impl<I: IntoIterator> IntoWiwiIter for I {
	type Iter = IterAdapter<I::IntoIter>;
	fn into_wiwi_iter(self) -> IterAdapter<I::IntoIter> {
		IterAdapter { inner: self.into_iter() }
	}
}

/// Allows any wiwi iter, or anything capable of converting to a wiwi iter
/// (ie. implements [`IntoIter`]), to convert to an std iterator. There is a
/// blanket implementation provided, so it is available for all wiwi iters.
pub trait IntoStdIterator {
	type Iterator: Iterator;
	fn into_std_iterator(self) -> Self::Iterator;
}

impl<I: IntoIter> IntoStdIterator for I {
	type Iterator = IterAdapter<I::Iter>;
	fn into_std_iterator(self) -> IterAdapter<I::Iter> {
		IterAdapter { inner: self.into_iter() }
	}
}
