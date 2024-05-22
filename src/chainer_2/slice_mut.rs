use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::MaybeUninit;
use super::SliceRefChain;

#[repr(transparent)]
pub struct SliceMutChain<'h, T> {
	inner: &'h mut [T]
}

impl<'h, T> SliceMutChain<'h, T> {
	pub fn as_slice(&self) -> &[T] {
		self.inner
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		self.inner
	}

	pub fn as_slice_ref_chainer(&'h self) -> SliceRefChain<'h, T> {
		(*self.inner).into()
	}

	pub fn into_inner(self) -> &'h mut [T] {
		self.inner
	}
}

impl<'h, T> SliceMutChain<'h, T> {
	pub fn len(self, out: &mut usize) -> Self {
		self.len_uninit(out.to_maybeuninit_mut())
	}

	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.len());
		self
	}
}

impl<'h, T> From<&'h mut [T]> for SliceMutChain<'h, T> {
	fn from(value: &'h mut [T]) -> Self {
		Self { inner: value }
	}
}
