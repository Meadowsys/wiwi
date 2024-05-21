use super::{ IntoIter, Iter, SizeHint, SizeHintBound };

pub struct RepeatPerItem<I, T> {
	iter: I,
	/// amount of times to emit each item
	count: usize,
	item: Option<Option<T>>,
	/// amount of times left to emit current item
	remaining_count: usize
}

impl<I, T> RepeatPerItem<I, T>
where
	I: Iter<Item = T>,
	T: Clone
{
	/// Called by [`Iter::repeat_per_item`]
	pub(super) fn new(iter: I, count: usize) -> Self {
		let item = if count == 0 {
			// this is this iter's exhausted state
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

	/// Consumes `self` and returns the underlying iterator.
	pub fn into_inner(self) -> I {
		self.iter
	}
}

impl<I, T> Iter for RepeatPerItem<I, T>
where
	I: Iter<Item = T>,
	T: Clone
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
				// triggers first branch
				self.item = None;
				item
			}

			Some(Some(_)) => {
				// special case for 1
				// lets just leave the stored one in there,
				// so it keeps triggering this branch
				// except for None, we'll update it to trigger exhausted branch

				let item = self.iter.next();
				if item.is_none() { self.item = Some(None) }
				item
			}
		}
	}

	fn size_hint(&self) -> SizeHint {
		use SizeHintBound::*;

		let (lower, upper) = self.iter.size_hint().split();
		let hint = SizeHint::new();

		let hint = match lower {
			HardBound { bound } => unsafe { hint.with_lower_hard_bound(bound * self.count) }
			Estimate { estimate } => { hint.with_lower_estimate(estimate * self.count) }
			Unknown => { hint.with_lower_unknown() }
		};

		match upper {
			HardBound { bound } => unsafe { hint.with_upper_hard_bound(bound * self.count) }
			Estimate { estimate } => { hint.with_upper_estimate(estimate * self.count) }
			Unknown => { hint.with_upper_unknown() }
		}
	}
}
