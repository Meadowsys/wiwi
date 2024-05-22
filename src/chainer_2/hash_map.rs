use std::collections::HashMap;
use std::hash::RandomState;

#[repr(transparent)]
pub struct HashMapChain<K, V, S = RandomState> {
	inner: HashMap<K, V, S>
}

impl<K, V, S> From<HashMap<K, V, S>> for HashMapChain<K, V, S> {
	fn from(value: HashMap<K, V, S>) -> Self {
		Self { inner: value }
	}
}
