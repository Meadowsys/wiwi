#[repr(transparent)]
pub struct ArrayChain<T, const N: usize> {
	inner: [T; N]
}

impl<T, const N: usize> From<[T; N]> for ArrayChain<T, N> {
	fn from(value: [T; N]) -> Self {
		Self { inner: value }
	}
}
