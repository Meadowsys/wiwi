use crate::prelude_std::*;

#[repr(transparent)]
pub struct GenericChain<T> {
	inner: T
}

impl<T> GenericChain<T> {
	#[inline]
	pub fn new(value: T) -> Self {
		Self { inner: value }
	}

	#[inline]
	pub fn with_inner<F, Void>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut T) -> Void
	{
		let _void = f(&mut self.inner);
		self
	}

	#[inline]
	pub fn into_inner(self) -> T {
		self.inner
	}
}
