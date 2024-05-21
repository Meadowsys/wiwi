#[repr(transparent)]
pub struct ArrayRefChain<T, const N: usize> {
	inner: [T; N]
}
