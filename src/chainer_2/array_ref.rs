#[repr(transparent)]
pub struct ArrayRefChain<T, const N: usize> {
	inner: [T; N]
}

impl<'h, T, const N: usize> From<&'h [T; N]> for &'h ArrayRefChain<T, N> {
	fn from(value: &'h [T; N]) -> Self {
		unsafe { &*(value as *const [T; N] as *const ArrayRefChain<T, N>) }
	}
}
