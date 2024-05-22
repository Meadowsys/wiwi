#[repr(transparent)]
pub struct StrRefChain<'h> {
	inner: &'h str
}
