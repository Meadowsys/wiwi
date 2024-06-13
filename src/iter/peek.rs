use super::{ Iter, SizeHintOld, SizeHintBoundOld };

pub trait Peekable: Iter {
	type PeekItem;
	fn peek(&mut self) -> Option<&Self::PeekItem>;
}

pub struct Peek<I, T> {
	iter: I,
	peeked: Option<Option<T>>
}

impl<I, T> Peek<I, T>
where
	I: Iter<Item = T>
{
	pub(super) fn new(iter: I) -> Self {
		Self { iter, peeked: None }
	}

	pub fn into_inner(self) -> (I, Option<Option<T>>) {
		let Self { iter, peeked } = self;
		(iter, peeked)
	}
}

impl<I, T> Iter for Peek<I, T>
where
	I: Iter<Item = T>
{
	type Item = I::Item;

	fn next(&mut self) -> Option<I::Item> {
		self.peeked.take().unwrap_or_else(|| self.iter.next())
	}

	fn _size_hint_old(&self) -> SizeHintOld {
		use SizeHintBoundOld::*;

		let peeked = self.peeked.is_some() as usize;
		let (lower, upper) = self.iter._size_hint_old().split();
		let hint = SizeHintOld::new();

		let hint = match lower {
			HardBound { bound } => unsafe { hint.with_lower_hard_bound(bound + peeked) }
			Estimate { estimate } => { hint.with_lower_estimate(estimate + peeked) }
			Unknown => { hint.with_lower_unknown() }
		};

		match upper {
			HardBound { bound } => unsafe { hint.with_upper_hard_bound(bound + peeked) }
			Estimate { estimate } => { hint.with_upper_estimate(estimate + peeked) }
			Unknown => { hint.with_upper_unknown() }
		}
	}
}

impl<I, T> Peekable for Peek<I, T>
where
	I: Iter<Item = T>,
{
	type PeekItem = I::Item;

	fn peek(&mut self) -> Option<&I::Item> {
		self.peeked.get_or_insert_with(|| self.iter.next()).as_ref()
	}
}

#[cfg(test)]
mod tests {
	use crate::iter::IntoIter;
	use super::*;

	#[test]
	fn peek() {
		let mut iter = vec![1, 2, 3]
			.into_wiwi_iter()
			.peekable();

		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(3) });
		assert_eq!(iter.peek(), Some(&1));
		assert_eq!(iter.peek(), Some(&1));
		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(3) });
		assert_eq!(iter.next(), Some(1));

		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(2) });
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(2) });
		assert_eq!(iter.next(), Some(2));

		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(1) });
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		// ... perhaps a bit excessive with the peeks?
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(1) });
		assert_eq!(iter.next(), Some(3));

		assert_eq!(iter.size_hint(), unsafe { SizeHint::hard_bound(0) });
		assert_eq!(iter.peek(), None);
		assert_eq!(iter.peek(), None);
		assert_eq!(iter.peek(), None);
		assert_eq!(iter.peek(), None);
		assert_eq!(iter.next(), None);
	}
}
