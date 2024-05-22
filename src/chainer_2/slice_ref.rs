use std::mem::size_of;
use std::slice;

#[repr(transparent)]
pub struct SliceRefChain<'h, T> {
	inner: &'h [T]
}

impl<'h, T> SliceRefChain<'h, T> {
	pub fn as_slice(&self) -> &[T] {
		self.inner
	}

	pub fn into_inner(self) -> &'h [T] {
		self.inner
	}
}

impl<'h, T, const N: usize> SliceRefChain<'h, [T; N]> {
	pub fn flatten(&self) -> SliceRefChain<'h, T> {
		// taken from std's flatten fn
		// TODO: use SizedTypeProperties or slice `flatten`, whichever gets stabilised first
		let len = if size_of::<T>() == 0 {
			self.inner.len()
				.checked_mul(N)
				.expect("slice len overflow")
		} else {
			// TODO: wait until 1.79 when this is stabilised
			// unsafe { self.inner.len().unchecked_mul(N) }

			self.inner.len() * N
		};

		let ptr = self as *const SliceRefChain<[T; N]> as *const T;
		unsafe { slice::from_raw_parts(ptr, len).into() }
	}
}

impl<'h, T> From<&'h [T]> for SliceRefChain<'h, T> {
	fn from(value: &'h [T]) -> Self {
		Self { inner: value }
	}
}
