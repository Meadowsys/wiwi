#[repr(transparent)]
pub struct VecChain<T> {
	inner: Vec<T>
}
