use std::mem::MaybeUninit;
use std::slice;
use super::ToMaybeUninit as _;

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct SliceMutChain<T> {
	inner: [T]
}

impl<T> SliceMutChain<T> {
	pub fn len(&mut self, out: &mut usize) -> &mut Self {
		self.len_uninit(out.to_maybeuninit_mut())
	}

	pub fn len_uninit(&mut self, out: &mut MaybeUninit<usize>) -> &mut Self {
		out.write(self.inner.len());
		self
	}

	pub fn is_empty(&mut self, out: &mut bool) -> &mut Self {
		self.is_empty_uninit(out.to_maybeuninit_mut())
	}

	pub fn is_empty_uninit(&mut self, out: &mut MaybeUninit<bool>) -> &mut Self {
		out.write(self.inner.is_empty());
		self
	}

	pub fn with_first<F>(&mut self, f: F) -> &mut Self
	where
		F: FnOnce(Option<&mut T>)
	{
		f(self.inner.first_mut());
		self
	}

	// TODO: more (see SliceBoxedChain)
}

// TODO: [AsciiChar] as_str, as_bytes

impl<T, const N: usize> SliceMutChain<[T; N]> {
	pub fn flatten(&mut self) -> &mut SliceMutChain<T> {
		let len = self.inner.len() * N;
		let ptr = self as *mut SliceMutChain<[T; N]> as *mut T;
		unsafe { slice::from_raw_parts_mut(ptr, len).into() }
	}
}

impl<T> From<&mut [T]> for &mut SliceMutChain<T> {
	fn from(value: &mut [T]) -> Self {
		unsafe { &mut *(value as *mut [T] as *mut SliceMutChain<T>) }
	}
}

// TODO: trait impls (see boxed slice chainer)
