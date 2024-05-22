#[repr(transparent)]
pub struct SliceRefChain<'h, T> {
	inner: &'h [T]
}

impl<'h, T> SliceRefChain<'h, T> {
	pub fn as_slice(&'h self) -> &'h [T] {
		self.inner
	}

	pub fn into_inner(self) -> &'h [T] {
		self.inner
	}
}

impl<'h, T> From<&'h [T]> for SliceRefChain<'h, T> {
	fn from(value: &'h [T]) -> Self {
		Self { inner: value }
	}
}
