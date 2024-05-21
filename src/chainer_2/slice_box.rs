#[repr(transparent)]
pub struct SliceBoxChain<T> {
	inner: Box<[T]>
}
