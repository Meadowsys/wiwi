use super::{ IntoIter, Iter };

pub struct RepeatPerItem<T, I> {
	iter: I,
	/// amount of times to emit each item
	count: usize,
	item: Option<Option<T>>,
	/// amount of times left to emit current item
	remaining_count: usize
}

impl<T, I> RepeatPerItem<T, I>
where
	T: Clone,
	I: Iter<Item = T>
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
}

impl<T, I> RepeatPerItem<T, I> {
	/// Consumes `self` and returns the underlying iterator.
	pub fn into_inner(self) -> I {
		self.iter
	}
}

impl<T, I> Iter for RepeatPerItem<T, I>
where
	T: Clone,
	I: Iter<Item = T>
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

	fn size_hint(&self) -> (Option<usize>, Option<usize>) {
		let (lower, upper) = self.iter.size_hint();

		// TODO: we have not specified what the extreme values mean
		// also, make custom enum for size hint!
		let lower = lower.map(|l| l.saturating_mul(self.count));
		let upper = upper.map(|u| u.saturating_mul(self.count));

		(lower, upper)
	}
}
