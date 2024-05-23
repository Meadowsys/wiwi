use super::{ ArrayRefChain, SliceRefChain, SliceMutChain };

#[repr(transparent)]
pub struct ArrayMutChain<'h, T, const N: usize> {
	inner: &'h mut [T; N]
}

impl<'h, T, const N: usize> ArrayMutChain<'h, T, N> {
	pub fn as_array(&'h self) -> &'h [T; N] {
		self.inner
	}

	pub fn as_mut_array(&'h mut self) -> &'h mut [T; N] {
		self.inner
	}

	pub fn as_slice(&'h self) -> &'h [T] {
		self.inner
	}

	pub fn as_mut_slice(&'h mut self) -> &'h mut [T] {
		self.inner
	}

	pub fn as_array_ref_chainer(&'h self) -> ArrayRefChain<'h, T, N> {
		(&*self.inner).into()
	}

	pub fn as_slice_ref_chainer(&'h self) -> SliceRefChain<'h, T> {
		(self.inner as &[T]).into()
	}

	pub fn as_slice_mut_chainer(&'h mut self) -> SliceMutChain<'h, T> {
		(self.inner as &mut [T]).into()
	}

	pub fn into_inner(self) -> &'h mut [T; N] {
		self.inner
	}
}

impl<'h, T, const N: usize> From<&'h mut [T; N]> for ArrayMutChain<'h, T, N> {
	fn from(value: &'h mut [T; N]) -> Self {
		Self { inner: value }
	}
}
