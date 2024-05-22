use std::collections::HashSet;
use std::hash::RandomState;

#[repr(transparent)]
pub struct HashSetChain<T, S = RandomState> {
	inner: HashSet<T, S>
}

impl<T, S> From<HashSet<T, S>> for HashSetChain<T, S> {
	fn from(value: HashSet<T, S>) -> Self {
		Self { inner: value }
	}
}
