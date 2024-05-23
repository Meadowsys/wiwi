use std::collections::VecDeque;
use super::{ SliceRefChain, SliceMutChain };

#[repr(transparent)]
pub struct VecDequeChain<T> {
	inner: VecDeque<T>
}

// TODO: eventually ref/mut versions

impl<T> VecDequeChain<T> {
	pub fn new() -> Self {
		VecDeque::new().into()
	}

	pub fn with_capacity(capacity: usize) -> Self {
		VecDeque::with_capacity(capacity).into()
	}
}

impl<T> VecDequeChain<T> {
	pub fn as_mut_contiguous(&mut self) -> &mut [T] {
		self.inner.make_contiguous()
	}

	pub fn as_vec_deque(&self) -> &VecDeque<T> {
		&self.inner
	}

	pub fn as_mut_vec_deque(&mut self) -> &mut VecDeque<T> {
		&mut self.inner
	}

	pub fn as_slices(&self) -> (&[T], &[T]) {
		self.inner.as_slices()
	}

	pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
		self.inner.as_mut_slices()
	}

	pub fn as_slice_ref_chainers(&self) -> (SliceRefChain<T>, SliceRefChain<T>) {
		let (s1, s2) = self.as_slices();
		(s1.into(), s2.into())
	}

	pub fn as_slice_mut_chainers(&mut self) -> (SliceMutChain<T>, SliceMutChain<T>) {
		let (s1, s2) = self.as_mut_slices();
		(s1.into(), s2.into())
	}

	pub fn into_inner(self) -> VecDeque<T> {
		self.inner
	}
}

impl<T> From<VecDeque<T>> for VecDequeChain<T> {
	fn from(value: VecDeque<T>) -> Self {
		Self { inner: value }
	}
}
