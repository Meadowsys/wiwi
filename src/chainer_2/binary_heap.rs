use std::collections::BinaryHeap;

#[repr(transparent)]
pub struct BinaryHeapChain<T> {
	inner: BinaryHeap<T>
}

impl<T> From<BinaryHeap<T>> for BinaryHeapChain<T> {
	fn from(value: BinaryHeap<T>) -> Self {
		Self { inner: value }
	}
}
