use std::mem::MaybeUninit;
use super::{ ArrayRefChain, ArrayMutChain, SliceRefChain, SliceMutChain };

#[repr(transparent)]
pub struct ArrayChain<T, const N: usize> {
	inner: [T; N]
}

impl<T, const N: usize> ArrayChain<T, N> {
	pub fn new_uninit() -> ArrayChain<MaybeUninit<T>, N> {
		unsafe {
			MaybeUninit::<[MaybeUninit<T>; N]>::uninit()
				.assume_init()
				.into()
		}
	}

	pub fn new_zeroed() -> ArrayChain<MaybeUninit<T>, N> {
		unsafe {
			MaybeUninit::<[MaybeUninit<T>; N]>::zeroed()
				.assume_init()
				.into()
		}
	}
}

impl<T, const N: usize> ArrayChain<T, N> {
	pub fn as_array(&self) -> &[T; N] {
		&self.inner
	}

	pub fn as_mut_array(&mut self) -> &mut [T; N] {
		&mut self.inner
	}

	pub fn as_slice(&self) -> &[T] {
		&self.inner
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		&mut self.inner
	}

	pub fn as_array_ref_chainer(&self) -> ArrayRefChain<T, N> {
		(&self.inner).into()
	}

	pub fn as_array_mut_chainer(&mut self) -> ArrayMutChain<T, N> {
		(&mut self.inner).into()
	}

	pub fn as_slice_ref_chainer(&self) -> SliceRefChain<T> {
		(&self.inner as &[T]).into()
	}

	pub fn as_slice_mut_chainer(&mut self) -> SliceMutChain<T> {
		(&mut self.inner as &mut [T]).into()
	}

	pub fn into_inner(self) -> [T; N] {
		self.inner
	}
}

impl<T, const N: usize> From<[T; N]> for ArrayChain<T, N> {
	fn from(value: [T; N]) -> Self {
		Self { inner: value }
	}
}
