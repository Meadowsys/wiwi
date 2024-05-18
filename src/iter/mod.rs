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

pub trait Iter {
	type Item;

	fn next(&mut self) -> Option<Self::Item>;

	fn size_hint(&self) -> (Option<usize>, Option<usize>) {
		(None, None)
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
