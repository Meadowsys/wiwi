use super::SliceRefChain;

#[repr(transparent)]
pub struct SliceMutChain<'h, T> {
	inner: &'h mut [T]
}

impl<'h, T> SliceMutChain<'h, T> {
	pub fn as_slice(&'h self) -> &'h [T] {
		self.inner
	}

	pub fn as_mut_slice(&'h mut self) -> &'h mut [T] {
		self.inner
	}

	pub fn as_slice_ref_chainer(&'h self) -> SliceRefChain<'h, T> {
		(*self.inner).into()
	}

	pub fn into_inner(self) -> &'h mut [T] {
		self.inner
	}
}

impl<'h, T> From<&'h mut [T]> for SliceMutChain<'h, T> {
	fn from(value: &'h mut [T]) -> Self {
		Self { inner: value }
	}
}
