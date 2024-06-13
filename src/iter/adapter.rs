//! Adapter traits and [`IterAdapter`], to aid in converting between and using std
//! [`Iterator`]s as wiwi [`Iter`]s, and vice versa

use super::{ DoubleEndedIter, IntoIter, Iter, SizeHintOld };

/// An adapter that wraps either a std [`Iterator`] or a wiwi [`Iter`], and then
/// acts as the other. Basically, with every layer you layer it, it flip flops
/// back and forth lol
///
/// If you want to convert one type of iter to the other, you're probably looking
/// for [`convert_std_into_wiwi_iter`](IntoWiwiIter::convert_std_into_wiwi_iter)
/// or [`convert_wiwi_into_std_iterator`](IntoStdIterator::convert_wiwi_into_std_iterator).
///
/// If you want to borrow one type of iter as the other, you're probably looking
/// for [`borrow_std_as_wiwi_iter`](AsWiwiIter::borrow_std_as_wiwi_iter)
/// or [`borrow_wiwi_as_std_iterator`](AsStdIterator::borrow_wiwi_as_std_iterator)
#[repr(transparent)]
pub struct IterAdapter<I> {
	inner: I
}

impl<I: Iter> Iterator for IterAdapter<I> {
	type Item = I::Item;
	fn next(&mut self) -> Option<I::Item> {
		self.inner.next()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner._size_hint_old().into_std_hint()
	}
}

impl<I: Iterator> Iter for IterAdapter<I> {
	type Item = I::Item;
	fn next(&mut self) -> Option<I::Item> {
		self.inner.next()
	}

	fn _size_hint_old(&self) -> SizeHintOld {
		let (lower, upper) = self.inner.size_hint();

		let bound = SizeHintOld::new()
			.with_lower_estimate(lower);

		if let Some(upper) = upper {
			bound.with_upper_estimate(upper)
		} else {
			bound.with_upper_unknown()
		}
	}
}

impl<I: DoubleEndedIter> DoubleEndedIterator for IterAdapter<I> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.inner.next_back()
	}
}

impl<I: DoubleEndedIterator> DoubleEndedIter for IterAdapter<I> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.inner.next_back()
	}
}

/// Allows any std iterator to be borrowed as a wiwi iter. There is a
/// blanket implementation provided, so it is available for all std iterators.
pub trait AsWiwiIter<'h> {
	type Iter: Iter + 'h;
	/// Borrow the std iterator as a wiwi iter.
	fn borrow_std_as_wiwi_iter(&'h mut self) -> Self::Iter;
}

impl<'h, I: Iterator + 'h> AsWiwiIter<'h> for I {
	type Iter = IterAdapter<&'h mut I>;
	fn borrow_std_as_wiwi_iter(&'h mut self) -> IterAdapter<&'h mut Self> {
		IterAdapter { inner: self }
	}
}

/// Allows any wiwi iter to be borrowed as an std iterator. There is a
/// blanket implementation provided, so it is available for all wiwi iters.
pub trait AsStdIterator<'h> {
	type Iterator: Iterator + 'h;
	/// Borrow the wiwi iter as an std iterator.
	fn borrow_wiwi_as_std_iterator(&'h mut self) -> Self::Iterator;
}

impl<'h, I: Iter + 'h> AsStdIterator<'h> for I {
	type Iterator = IterAdapter<&'h mut I>;
	fn borrow_wiwi_as_std_iterator(&'h mut self) -> IterAdapter<&'h mut Self> {
		IterAdapter { inner: self }
	}
}

/// Allows any std iterator, or anything capable of converting to an std iterator
/// (ie. implements [`IntoIterator`]), to convert to a wiwi iter. There is a blanket
/// implementation provided, so it is available for all std iterators.
pub trait IntoWiwiIter {
	type Iter: Iter;
	/// Converts the std iterator into a wiwi iter.
	fn convert_std_into_wiwi_iter(self) -> Self::Iter;
}

impl<I: IntoIterator> IntoWiwiIter for I {
	type Iter = IterAdapter<I::IntoIter>;
	fn convert_std_into_wiwi_iter(self) -> IterAdapter<I::IntoIter> {
		IterAdapter { inner: self.into_iter() }
	}
}

/// Allows any wiwi iter, or anything capable of converting to a wiwi iter
/// (ie. implements [`IntoIter`]), to convert to an std iterator. There is a
/// blanket implementation provided, so it is available for all wiwi iters.
pub trait IntoStdIterator {
	type Iterator: Iterator;
	/// Converts the wiwi iter into a std iterator.
	fn convert_wiwi_into_std_iterator(self) -> Self::Iterator;
}

impl<I: IntoIter> IntoStdIterator for I {
	type Iterator = IterAdapter<I::Iter>;
	fn convert_wiwi_into_std_iterator(self) -> IterAdapter<I::Iter> {
		IterAdapter { inner: self.into_wiwi_iter() }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn adapter() {
		let vec = vec![1, 2, 3]
			.into_wiwi_iter()
			.into_wiwi_iter()
			.convert_wiwi_into_std_iterator()
			.convert_std_into_wiwi_iter()
			.borrow_wiwi_as_std_iterator()
			.borrow_std_as_wiwi_iter()
			.into_wiwi_iter()
			.convert_wiwi_into_std_iterator()
			// many indirection layers later...
			.collect::<Vec<_>>();

		assert_eq!(vec, [1, 2, 3]);
	}
}
