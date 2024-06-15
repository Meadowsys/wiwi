use super::{ IntoIter, Iter, SizeHintBound, SizeHintImpl, SizeHintInner, SizeHintMarker };

pub struct RepeatPerItem<I: Iter> {
	iter: I,
	/// amount of times to emit each item
	count: usize,
	item: Option<Option<I::Item>>,
	/// amount of times left to emit current item
	remaining_count: usize
}

impl<I> RepeatPerItem<I>
where
	I: Iter,
	I::Item: Clone
{
	/// Called by [`Iter::repeat_per_item`]
	pub(super) fn new(iter: I, count: usize) -> Self {
		let item = if count == 0 {
			// marks inner iter as "exhausted"
			Some(None)
		} else {
			None
		};

		Self {
			iter,
			count,
			item,
			remaining_count: 0
		}
	}

	/// Consumes `self` and returns the underlying iter.
	pub fn into_inner(self) -> (I, Option<Option<I::Item>>) {
		(self.iter, self.item)
	}
}

impl<I> Iter for RepeatPerItem<I>
where
	I: Iter,
	I::Item: Clone
{
	type Item = I::Item;

	fn next(&mut self) -> Option<I::Item> {
		match &mut self.item {
			None => {
				// need to get next item

				let item = self.iter.next();

				// we're not checking for 1 here, which feels like a performance penalty
				// paid for an minority case, including the usual 2+ count case
				self.item = Some(item.clone());
				self.remaining_count = self.count - 1;

				item
			}

			Some(None) => {
				// inner iter is exhausted
				// we will never call `next()` on it again
				None
			}

			Some(Some(item)) if self.remaining_count > 1 => {
				// we'll need the item again to clone it at least once more
				self.remaining_count -= 1;
				Some(item.clone())
			}

			Some(item @ Some(_)) if self.remaining_count == 1 => {
				// last iteration for this element
				let item = item.take();
				// triggers `None` branch on next iteration
				self.item = None;
				// for size_hint
				self.remaining_count = 0;
				item
			}

			Some(Some(_)) => {
				// special case for 1, only way remaining_count will be 0

				// we don't return again since there's 0 iters remaining for this elem
				// drop it, iter again
				self.item = None;
				self.next()
			}
		}
	}

	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		use SizeHintBound::*;
		use SizeHintInner::*;

		macro_rules! count {
			($count:ident) => { ($count * self.count) + self.remaining_count }
		}

		match self.iter.size_hint().into_inner() {
			Unknown => { SizeHintImpl::unknown() }

			Upper { bound: Hard { count } } => { SizeHintImpl::upper_hard(count!(count)) }
			Upper { bound: Estimate { count } } => { SizeHintImpl::upper_estimate(count!(count)) }

			Lower { bound: Hard { count } } => { SizeHintImpl::lower_hard(count!(count)) }
			Lower { bound: Estimate { count } } => { SizeHintImpl::lower_estimate(count!(count)) }

			Single { bound: Hard { count } } => { SizeHintImpl::hard(count!(count)) }
			Single { bound: Estimate { count } } => { SizeHintImpl::estimate(count!(count)) }

			Range { lower: Estimate { count: cl }, upper: Estimate { count: cu } } => { SizeHintImpl::range_estimate(count!(cl), count!(cu)) }
			Range { lower: Estimate { count: cl }, upper: Hard { count: cu } } => { SizeHintImpl::range_lestimate_uhard(count!(cl), count!(cu)) }
			Range { lower: Hard { count: cl }, upper: Estimate { count: cu } } => { SizeHintImpl::range_lhard_uestimate(count!(cl), count!(cu)) }
			Range { lower: Hard { count: cl }, upper: Hard { count: cu } } => { SizeHintImpl::range_hard(count!(cl), count!(cu)) }
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::iter::{ IntoStdIterator, IntoWiwiIter };
	use super::*;

	#[test]
	fn repeat_per_item() {
		let vec = vec![1, 2, 3]
			.into_wiwi_iter()
			.map(|i| i * 9)
			.repeat_per_item(2)
			// TODO: use our own
			.convert_wiwi_into_std_iterator()
			.collect::<Vec<_>>();
		assert_eq!(vec, [9, 9, 18, 18, 27, 27]);
	}

	#[test]
	fn into_inner() {
		let iter = vec![1, 2, 3].into_wiwi_iter().repeat_per_item(2);

		let (iter, item) = iter.into_inner();
		// TODO: use our own
		let iter = iter.convert_wiwi_into_std_iterator().collect::<Vec<_>>();
		assert_eq!(item, None);
		assert_eq!(iter, [1, 2, 3]);

		let mut iter = vec![1, 2, 3].into_wiwi_iter().repeat_per_item(2);
		let _ = iter.next();
		let _ = iter.next();
		let _ = iter.next();

		let (iter, item) = iter.into_inner();
		// TODO: use our own
		let iter = iter.convert_wiwi_into_std_iterator().collect::<Vec<_>>();
		assert_eq!(item, Some(Some(2)));
		assert_eq!(iter, [3]);
	}

	#[test]
	fn size_hint() {
		let mut iter = vec![1, 2, 3]
			.into_wiwi_iter()
			.repeat_per_item(2);

		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(6) });
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(5) });
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(4) });
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(3) });
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(2) });
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(1) });
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(0) });
		assert_eq!(iter.next(), None);

		let mut iter = vec![1, 2, 3]
			.into_iter()
			.convert_std_into_wiwi_iter()
			.repeat_per_item(2);

		// same as above, but estimate only
		// (since there's an std iterator adapter in there)
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(6) });
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(5) });
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(4) });
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(3) });
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(2) });
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(1) });
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(0) });
		assert_eq!(iter.next(), None);
	}
}
