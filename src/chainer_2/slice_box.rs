use std::mem::MaybeUninit;
use super::VecChain;

#[repr(transparent)]
pub struct SliceBoxChain<T> {
	inner: Box<[T]>
}

impl<T> SliceBoxChain<T> {
	pub fn new_uninit(len: usize) -> SliceBoxChain<MaybeUninit<T>> {
		unsafe {
			VecChain::with_capacity(len)
				.set_len(len)
				.into_boxed_slice()
				.into()
		}
	}

	pub fn new_zeroed(len: usize) -> SliceBoxChain<MaybeUninit<T>> {
		let mut this = Self::new_uninit(len);
		unsafe { this.inner.as_mut_ptr().write_bytes(0, len) }
		this
	}
}

impl<T> From<Box<[T]>> for SliceBoxChain<T> {
	fn from(value: Box<[T]>) -> Self {
		Self { inner: value }
	}
}
