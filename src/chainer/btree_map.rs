use std::collections::BTreeMap;

#[repr(transparent)]
pub struct BTreeMapChain<K, V> {
	inner: BTreeMap<K, V>
}

impl<K, V> From<BTreeMap<K, V>> for BTreeMapChain<K, V> {
	fn from(value: BTreeMap<K, V>) -> Self {
		Self { inner: value }
	}
}
