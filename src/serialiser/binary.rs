#![allow(dead_code, reason = "wip (todo remove me)")]

use crate::prelude_std::*;

pub trait Serialise<'h> {
	type Serialiser: Serialiser<'h>;

	fn serialiser(&'h self) -> Self::Serialiser;
}

pub trait Serialiser<'h>: Sized {}

pub trait Deserialise<'h>: Sized {
	type Error: std::error::Error + From<Error>;
}

/// Error type but it's blank (it will be filled soon™)
pub struct Error {
	inner: ErrorInner
}

enum ErrorInner {
	/// Something™ happened
	Something
}

impl<'h, T> Serialise<'h> for &T
where
	T: ?Sized + Serialise<'h>
{
	type Serialiser = T::Serialiser;

	#[inline]
	fn serialiser(&'h self) -> T::Serialiser {
		T::serialiser(self)
	}
}

impl<'h, T> Serialise<'h> for &mut T
where
	T: ?Sized + Serialise<'h>
{
	type Serialiser = T::Serialiser;

	#[inline]
	fn serialiser(&'h self) -> T::Serialiser {
		T::serialiser(self)
	}
}

impl<'h, T> Serialise<'h> for Box<T>
where
	T: ?Sized + Serialise<'h>
{
	type Serialiser = T::Serialiser;

	#[inline]
	fn serialiser(&'h self) -> T::Serialiser {
		T::serialiser(self)
	}
}
