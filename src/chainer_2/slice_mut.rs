#[repr(transparent)]
pub struct SliceMutChain<T> {
	inner: [T]
}
