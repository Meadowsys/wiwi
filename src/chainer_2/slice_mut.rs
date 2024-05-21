#[repr(transparent)]
pub struct SliceMutChain<'h, T> {
	inner: &'h mut [T]
}

impl<'h, T> From<&'h mut [T]> for SliceMutChain<'h, T> {
	fn from(value: &'h mut [T]) -> Self {
		Self { inner: value }
	}
}
