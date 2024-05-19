#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct ArrayMutChain<T, const N: usize> {
	inner: [T; N]
}
