use super::IntoChainer;

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct ArrayRefChain<T, const N: usize> {
	inner: [T; N]
}

impl<'h, T, const N: usize> IntoChainer for &'h [T; N] {
	type Chain = &'h ArrayRefChain<T, N>;
	fn into_chainer(self) -> &'h ArrayRefChain<T, N> {
		self.into()
	}
}

impl<T, const N: usize> From<&[T; N]> for &ArrayRefChain<T, N> {
	fn from(value: &[T; N]) -> Self {
		unsafe { &*(value as *const [T; N] as *const ArrayRefChain<T, N>) }
	}
}
