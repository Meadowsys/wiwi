#[repr(transparent)]
pub struct SliceRefChain<'h, T> {
	inner: &'h [T]
}

impl<'h, T> From<&'h [T]> for SliceRefChain<'h, T> {
	fn from(value: &'h [T]) -> Self {
		Self { inner: value }
	}
}
