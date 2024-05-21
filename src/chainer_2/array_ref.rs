#[repr(transparent)]
pub struct ArrayRefChain<'h, T, const N: usize> {
	inner: &'h [T; N]
}

impl<'h, T, const N: usize> From<&'h [T; N]> for ArrayRefChain<'h, T, N> {
	fn from(value: &'h [T; N]) -> Self {
		Self { inner: value }
	}
}
