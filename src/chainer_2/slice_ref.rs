#[repr(transparent)]
pub struct SliceRefChain<T> {
	inner: [T]
}

impl<'h, T> From<&'h [T]> for &'h SliceRefChain<T> {
	fn from(value: &'h [T]) -> Self {
		unsafe { &*(value as *const [T] as *const SliceRefChain<T>) }
	}
}
