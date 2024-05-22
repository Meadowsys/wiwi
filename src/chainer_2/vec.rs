use super::{ SliceRefChain, SliceMutChain };

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
