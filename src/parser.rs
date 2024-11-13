use crate::prelude_std::*;
use std::num::NonZero;

pub use stateful::ParserStateful;
pub use stateless::Parser;

pub mod stateful;
pub mod stateless;

pub struct Success<D, O> {
	pub output: O,
	pub data: D,
}

pub enum Error<E> {
	NotEnoughData { missing: Option<NonZero<usize>> },
	Error { error: E },
	Fatal { error: E }
}

pub type Result<D, O, E = ()> = std::result::Result<Success<D, O>, Error<E>>;

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
