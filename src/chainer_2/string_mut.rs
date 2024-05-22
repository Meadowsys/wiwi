#[repr(transparent)]
pub struct StringMutChain<'h> {
	inner: &'h mut String
}
