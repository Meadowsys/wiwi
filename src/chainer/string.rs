#[repr(transparent)]
pub struct StringChain {
	inner: String
}

impl From<String> for StringChain {
	fn from(value: String) -> Self {
		Self { inner: value }
	}
}
