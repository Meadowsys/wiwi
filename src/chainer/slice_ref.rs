use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::{ MaybeUninit, size_of };
use std::slice;
use super::IntoChainer;

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct SliceRefChain<T> {
	inner: [T]
}

/// Constructor functions
impl<T> SliceRefChain<T> {
	// from_ref, from_raw_parts, from_ptr_range, (nightly) range
}

/// Conversion functions
impl<T> SliceRefChain<T> {
	pub fn as_slice(&self) -> &[T] {
		&self.inner
	}
}

/// Chaining functions
impl<T> SliceRefChain<T> {
	pub fn len(&self, out: &mut usize) -> &Self {
		self.len_uninit(out.to_maybeuninit_mut())
	}

	pub fn len_uninit(&self, out: &mut MaybeUninit<usize>) -> &Self {
		out.write(self.inner.len());
		self
	}

	pub fn is_empty(&self, out: &mut bool) -> &Self {
		self.is_empty_uninit(out.to_maybeuninit_mut())
	}

	pub fn is_empty_uninit(&self, out: &mut MaybeUninit<bool>) -> &Self {
		out.write(self.inner.is_empty());
		self
	}

	pub fn first<'h>(&'h self, out: &mut Option<&'h T>) -> &Self {
		self.first_uninit(out.to_maybeuninit_mut())
	}

	pub fn first_uninit<'h>(&'h self, out: &mut MaybeUninit<Option<&'h T>>) -> &Self {
		out.write(self.inner.first());
		self
	}

	pub fn with_first<F>(&self, f: F) -> &Self
	where
		F: FnOnce(Option<&T>)
	{
		f(self.inner.first());
		self
	}

	// TODO: more (see SliceBoxedChain)
}

// TODO: [AsciiChar] as_str, as_bytes

impl<T, const N: usize> SliceRefChain<[T; N]> {
	pub fn flatten(&self) -> &SliceRefChain<T> {
		// taken from std's flatten fn
		// TODO: use SizedTypeProperties or slice `flatten`, whichever gets stabilised first
		let len = if size_of::<T>() == 0 {
			self.inner.len()
				.checked_mul(N)
				.expect("slice len overflow")
		} else {
			unsafe { self.inner.len().unchecked_mul(N) }
		};

		let ptr = self as *const SliceRefChain<[T; N]> as *const T;
		unsafe { slice::from_raw_parts(ptr, len).into() }
	}
}

impl<'h, T> IntoChainer for &'h [T] {
	type Chain = &'h SliceRefChain<T>;
	fn into_chainer(self) -> &'h SliceRefChain<T> {
		self.into()
	}
}

impl<T> From<&[T]> for &SliceRefChain<T> {
	fn from(value: &[T]) -> Self {
		unsafe { &*(value as *const [T] as *const SliceRefChain<T>) }
	}
}

// TODO: trait impls (see boxed slice chainer)
