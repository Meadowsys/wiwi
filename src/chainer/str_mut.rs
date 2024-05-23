#[repr(transparent)]
pub struct StrMutChain<'h> {
	inner: &'h mut str
}
