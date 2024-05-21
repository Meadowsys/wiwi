#[repr(transparent)]
pub struct ArrayChain<T, const N: usize> {
	inner: [T; N]
}
