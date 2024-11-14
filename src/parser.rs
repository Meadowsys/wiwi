use crate::prelude_std::*;
use std::num::NonZero;

pub use input::{ Input, Needle };
pub use stateful::ParserStateful;
pub use stateless::Parser;

pub mod input;
pub mod stateful;
pub mod stateless;

pub struct Success<I, O> {
	pub output: O,
	pub remaining_input: I,
}

pub enum Error<E> {
	NotEnoughData { missing: Option<NonZero<usize>> },
	Error { error: E },
	Fatal { error: E }
}

pub type Result<I, O, E = ()> = std::result::Result<Success<I, O>, Error<E>>;

pub type ParserPhantom<I, O, E = ()> = PhantomData<ParserPhantomImpl<I, O, E>>;

/// This is an implementation detail of [`ParserPhantom`]
///
/// This helps to enforce the trait bounds of [`Parser`]/[`ParserStateful`] in
/// [`ParserPhantom`] type, to work around the fact that type aliases don't
/// enforce their trait bounds for now, which is a known rust type checker limitation.
#[doc(hidden)]
pub struct ParserPhantomImpl<I, O, E>
where
	I: Input
{
	__inner: fn(I) -> Result<I, O, E>
}

impl<E> Error<E> {
	#[inline]
	fn from<EFrom>(error: Error<EFrom>) -> Self
	where
		EFrom: Into<E>
	{
		use self::Error::*;

		match error {
			NotEnoughData { missing } => { NotEnoughData { missing } }
			Error { error } => { Error { error: error.into() } }
			Fatal { error } => { Fatal { error: error.into() } }
		}
	}

	#[inline]
	fn into<EInto>(self) -> Error<EInto>
	where
		E: Into<EInto>
	{
		Error::from(self)
	}
}
