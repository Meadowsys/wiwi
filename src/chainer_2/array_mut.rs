#[repr(transparent)]
pub struct ArrayMutChain<'h, T, const N: usize> {
	inner: &'h mut [T; N]
}

impl<'h, T, const N: usize> From<&'h mut [T; N]> for ArrayMutChain<'h, T, N> {
	fn from(value: &'h mut [T; N]) -> Self {
		Self { inner: value }
	}
}
