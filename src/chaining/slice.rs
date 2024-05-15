// TODO: allocator param
#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct SliceChain<T> {
	inner: [T]
}

impl<T> From<&[T]> for &SliceChain<T> {
	fn from(value: &[T]) -> Self {
		unsafe { &*(value as *const [T] as *const SliceChain<T>) }
	}
}

impl<T> From<&mut [T]> for &mut SliceChain<T> {
	fn from(value: &mut [T]) -> Self {
		unsafe { &mut *(value as *mut [T] as *mut SliceChain<T>) }
	}
}

impl<T> AsRef<SliceChain<T>> for SliceChain<T> {
	fn as_ref(&self) -> &Self {
		self
	}
}

impl<T> AsMut<SliceChain<T>> for SliceChain<T> {
	fn as_mut(&mut self) -> &mut Self {
		self
	}
}

impl<T> AsRef<[T]> for SliceChain<T> {
	fn as_ref(&self) -> &[T] {
		&self.inner
	}
}

impl<T> AsMut<[T]> for SliceChain<T> {
	fn as_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}
}
