//! moo
//!
//! Deserialise implementations are done individually rather than a blanket impl.
//! This is because some things (like strings) can be and will benefit from zero
//! copy deserialisation (ie. deserialising to Cow::Borrowed), but other things
//! (ex. arrays of assorted compressed integers) cannot be. This hypothetical
//! blanket deser impl could be done by always serialising to the owned variant
//! (ie. deserialising to Cow::Owned), but types that can zero copy deserialise
//! lose out on the ability to.
//!
//! Rust! Specialisation when?

use super::internal_prelude::*;
use std::borrow::Cow;

impl<T> Serialise for Cow<'_, T>
where
	T: Serialise + ToOwned + ?Sized
{
	type Serialiser<'h> = CowSerialiser<T::Serialiser<'h>> where Self: 'h;

	#[inline]
	fn build_serialiser(&self) -> CowSerialiser<T::Serialiser<'_>> {
		CowSerialiser::new(T::build_serialiser(self))
	}
}

pub struct CowSerialiser<T> {
	inner: T
}

impl<T> CowSerialiser<T> {
	#[inline]
	fn new(ser: T) -> Self {
		Self { inner: ser }
	}
}

impl<'h, T: Serialiser<'h>> Serialiser<'h> for CowSerialiser<T> {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.inner.needed_capacity()
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		self.inner.serialise(buf)
	}
}
