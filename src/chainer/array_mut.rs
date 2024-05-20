use super::IntoChainer;

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct ArrayMutChain<T, const N: usize> {
	inner: [T; N]
}

impl<'h, T, const N: usize> IntoChainer for &'h mut [T; N] {
	type Chain = &'h mut ArrayMutChain<T, N>;
	fn into_chainer(self) -> &'h mut ArrayMutChain<T, N> {
		self.into()
	}
}

impl<T, const N: usize> From<&mut [T; N]> for &mut ArrayMutChain<T, N> {
	fn from(value: &mut [T; N]) -> Self {
		unsafe { &mut *(value as *mut [T; N] as *mut ArrayMutChain<T, N>) }
	}
}
