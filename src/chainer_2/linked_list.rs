use std::collections::LinkedList;

#[repr(transparent)]
pub struct LinkedListChain<T> {
	inner: LinkedList<T>
}

impl<T> From<LinkedList<T>> for LinkedListChain<T> {
	fn from(value: LinkedList<T>) -> Self {
		Self { inner: value }
	}
}
