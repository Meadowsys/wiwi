#[repr(transparent)]
pub struct ArrayMutChain<T, const N: usize> {
	inner: [T; N]
}

impl<'h, T, const N: usize> From<&'h mut [T; N]> for &'h mut ArrayMutChain<T, N> {
	fn from(inner: &'h mut [T; N]) -> Self {
		unsafe { &mut *(inner as *mut [T; N] as *mut ArrayMutChain<T, N>) }
	}
}
