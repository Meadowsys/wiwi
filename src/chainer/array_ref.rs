#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct ArrayRefChain<T, const N: usize> {
	inner: [T; N]
}
