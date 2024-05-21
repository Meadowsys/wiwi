#[repr(transparent)]
pub struct VecChain<T> {
	inner: Vec<T>
}

impl<T> From<Vec<T>> for VecChain<T> {
	fn from(value: Vec<T>) -> Self {
		Self { inner: value }
	}
}
