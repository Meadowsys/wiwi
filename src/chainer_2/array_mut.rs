#[repr(transparent)]
pub struct ArrayMutChain<T, const N: usize> {
	inner: [T; N]
}
