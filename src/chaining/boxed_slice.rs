use std::mem::MaybeUninit;

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct BoxedSliceChain<T> {
	inner: Box<[T]>
}

impl<T> BoxedSliceChain<T> {
	// TODO: nightly new_uninit_slice
	// TODO: nightly new_zeroed_slice
	// TODO: nightly try_new_uninit_slice
	// TODO: nightly try_new_zeroed_slice
}

// TODO: allocator param
impl<T> BoxedSliceChain<T> {
	// TODO: nightly new_uninit_slice_in
	// TODO: nightly new_zeroed_slice_in
}

// TODO: allocator param
impl<T> BoxedSliceChain<MaybeUninit<T>> {
	// TODO: nightly assume_init
}

// TODO: allocator param
impl<T> From<Box<[T]>> for BoxedSliceChain<T> {
	#[inline]
	fn from(inner: Box<[T]>) -> Self {
		Self { inner }
	}
}
