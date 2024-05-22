#[repr(transparent)]
pub struct StrBoxChain {
	inner: Box<str>
}
