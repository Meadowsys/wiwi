#[repr(transparent)]
pub struct SliceBoxChain<T> {
	inner: Box<[T]>
}

impl<T> From<Box<[T]>> for SliceBoxChain<T> {
	fn from(value: Box<[T]>) -> Self {
		Self { inner: value }
	}
}
