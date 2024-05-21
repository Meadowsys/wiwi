#[repr(transparent)]
pub struct SliceRefChain<T> {
	inner: [T]
}
