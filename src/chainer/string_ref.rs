#[repr(transparent)]
pub struct StringRefChain<'h> {
	inner: &'h String
}
