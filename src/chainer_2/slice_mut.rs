#[repr(transparent)]
pub struct SliceMutChain<T> {
	inner: [T]
}

impl<'h, T> From<&'h mut [T]> for &'h mut SliceMutChain<T> {
	fn from(value: &'h mut [T]) -> Self {
		unsafe { &mut *(value as *mut [T] as *mut SliceMutChain<T>) }
	}
}
