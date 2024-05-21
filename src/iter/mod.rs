mod adapter;
pub use adapter::{
	AsStdIterator,
	AsWiwiIter,
	IntoStdIterator,
	IntoWiwiIter,
	IterAdapter
};

mod empty;
pub use empty::{ empty, Empty };

mod from_fn;
pub use from_fn::{ from_fn, FromFn };

mod into_iter;
pub use into_iter::IntoIter;

mod repeat_per_item;
pub use repeat_per_item::RepeatPerItem;

mod size_hint;
pub use size_hint::{ SizeHint, SizeHintBound };

pub trait Iter {
	type Item;

	fn next(&mut self) -> Option<Self::Item>;

	fn size_hint(&self) -> SizeHint {
		SizeHint::unknown()
	}

	/// Takes every element emitted by the underlying iterator, and returns it
	/// `count` times via cloning.
	///
	/// If the count provided is 1, elements may be needlessly cloned.
	//  side note: I'm purposefully not specifying the exact behaviour
	///
	/// If the count provided is 0, the iterator will never be advanced, and the
	/// returned iterator will yield no elements.
	//  another side note: this... feels less expensive than checking for 0 every
	//  iteration and does match the behaviour that one may expect of asking to
	//  repeat every item 0 times lol (if everything comes out 0 times... nothing
	//  comes out), and feels better to me than panicking. I mean, technically
	//  it's not wrong to ask repeat every element 0 times...
	///
	/// This iterator is fused, meaning this iterator will always yield `None`
	/// once the underlying iter does, and will never advance it again.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::iter::{ IntoWiwiIter, Iter };
	/// let mut iter = [1, 2, 3]
	///    .into_wiwi_iter()
	///    .repeat_per_item(3);
	///
	/// assert_eq!(iter.next(), Some(1));
	/// assert_eq!(iter.next(), Some(1));
	/// assert_eq!(iter.next(), Some(1));
	///
	/// assert_eq!(iter.next(), Some(2));
	/// assert_eq!(iter.next(), Some(2));
	/// assert_eq!(iter.next(), Some(2));
	///
	/// assert_eq!(iter.next(), Some(3));
	/// assert_eq!(iter.next(), Some(3));
	/// assert_eq!(iter.next(), Some(3));
	///
	/// assert_eq!(iter.next(), None);
	/// ```
	///
	/// Passing in `0` for the count:
	///
	/// ```
	/// # use wiwi::iter::{ IntoWiwiIter, Iter };
	/// let mut iter = [1, 2, 3]
	///    .into_wiwi_iter()
	///    .repeat_per_item(0);
	///
	/// // Never going to get anything out of it...
	/// assert_eq!(iter.next(), None);
	/// assert_eq!(iter.next(), None);
	/// assert_eq!(iter.next(), None);
	///
	/// // ...but the original iter is indeed intact
	/// let mut orig_iter = iter.into_inner();
	/// assert_eq!(orig_iter.next(), Some(1));
	/// assert_eq!(orig_iter.next(), Some(2));
	/// assert_eq!(orig_iter.next(), Some(3));
	/// assert_eq!(orig_iter.next(), None);
	/// ```
	fn repeat_per_item(self, count: usize) -> RepeatPerItem<Self::Item, Self>
	where
		Self: Sized,
		Self::Item: Clone
	{
		RepeatPerItem::new(self, count)
	}

	/*
	next_chunk
	count
	last
	advance_by
	nth
	step_by
	chain
	zip
	intersperse
	intersperse_with
	map
	for_each
	filter
	filter_map
	enumerate
	peekable
	skip_while
	take_while
	map_while
	skip
	take
	scan
	flat_map
	flatten
	map_windows
	fuse
	inspect
	by_ref
	collect
	try_collect
	collect_into
	partition
	partition_in_place
	is_partitioned
	try_fold
	try_for_each
	fold
	reduce
	try_reduce
	all
	any
	find
	find_map
	try_find
	position
	rposition
	max
	min
	max_by_key
	max_by
	min_by_key
	min_by
	rev
	unzip
	copied
	cloned
	cycle
	array_chunks
	sum
	product
	cmp
	cmp_by
	partial_cmp
	partial_cmp_by
	eq
	eq_by
	ne
	lt
	le
	gt
	ge
	is_sorted/by/key

	traits
	product, sum
	extend, from/intoiter
	double ended iter
	exact size iter
	iter
	fused iter, trusted len
	in place iterable
	trusted fused/step
	unchecked iter
	trusted/step

	fns
	empty
	from_fn
	once/with
	repeat/with
	successors
	zip
	from_coroutine
	repeat_n
	*/
}

impl<I: Iter> Iter for &mut I {
	type Item = I::Item;
	fn next(&mut self) -> Option<I::Item> {
		(**self).next()
	}
}
