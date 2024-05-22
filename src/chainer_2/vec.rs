use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::{ ManuallyDrop, MaybeUninit };
use super::{ SliceBoxChain, SliceRefChain, SliceMutChain };

#[repr(transparent)]
pub struct VecChain<T> {
	inner: Vec<T>
}

impl<T> VecChain<T> {
	pub fn new() -> Self {
		Vec::new().into()
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity).into()
	}

	// TODO: try_with_capacity

	pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
		Vec::from_raw_parts(ptr, length, capacity).into()
	}

	pub fn as_slice(&self) -> &[T] {
		&self.inner
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		&mut self.inner
	}

	pub fn as_slice_ref_chainer(&self) -> SliceRefChain<T> {
		(*self.inner).into()
	}

	pub fn as_slice_mut_chainer(&mut self) -> SliceMutChain<T> {
		(&mut *self.inner).into()
	}

	pub fn as_vec(&self) -> &Vec<T> {
		&self.inner
	}

	pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
		&mut self.inner
	}

	pub fn into_inner(self) -> Vec<T> {
		self.inner
	}

	pub fn into_boxed_slice(self) -> Box<[T]> {
		self.inner.into_boxed_slice()
	}

	pub fn into_boxed_slice_chainer(self) -> SliceBoxChain<T> {
		self.into_boxed_slice().into()
	}
}

// TODO: for alloc param
impl<T> VecChain<T> {
	// TODO: new_in
	// TODO: with_capacity_in
	// TODO: try_with_capacity_in
	// TODO: from_raw_parts_in

	pub fn into_raw_parts(self) -> (*mut T, usize, usize) {
		// TODO: use std's version once stable

		let mut me = ManuallyDrop::new(self);

		let pointer = me.inner.as_mut_ptr();
		let length = me.inner.len();
		let capacity = me.inner.capacity();

		(pointer, length, capacity)
	}

	// TODO: into_raw_parts_with_alloc
}

impl<T> VecChain<T> {
	pub fn capacity(self, out: &mut usize) -> Self {
		self.capacity_uninit(out.to_maybeuninit_mut())
	}

	pub fn capacity_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.capacity());
		self
	}

	pub fn reserve(mut self, additional: usize) -> Self {
		self.inner.reserve(additional);
		self
	}

	pub fn reserve_exact(mut self, additional: usize) -> Self {
		self.inner.reserve_exact(additional);
		self
	}

	// TODO: try_reserve/exact

	pub fn shrink_to_fit(mut self) -> Self {
		self.inner.shrink_to_fit();
		self
	}

	pub fn shrink_to(mut self, min_capacity: usize) -> Self {
		self.inner.shrink_to(min_capacity);
		self
	}

	pub fn truncate(mut self, len: usize) -> Self {
		self.inner.truncate(len);
		self
	}
}

impl<T> VecChain<T> {
	pub unsafe fn set_len(mut self, new_len: usize) -> Self {
		self.inner.set_len(new_len);
		self
	}
}

impl<T> From<Vec<T>> for VecChain<T> {
	fn from(value: Vec<T>) -> Self {
		Self { inner: value }
	}
}
