extern crate wiwiwiwiwi;

use crate::prelude_std::*;
use self::private::AcceptDefaultSealed;

pub use wiwiwiwiwi::builder;

pub struct Uninit {
	__private: ()
}

pub struct Init {
	__private: ()
}

/// Trait for required items that can take [`None`] for convenience, because
/// they implement [`Default`]
pub trait AcceptDefault<T>
where
	Self: AcceptDefaultSealed<T>
{
	fn unwrap_or_default(self) -> T;
}

impl<T> AcceptDefault<T> for T {
	#[inline]
	fn unwrap_or_default(self) -> T {
		self
	}
}

impl<T> AcceptDefaultSealed<T> for T {}

impl<T> AcceptDefault<T> for Option<T>
where
	T: Default
{
	#[inline]
	fn unwrap_or_default(self) -> T {
		self.unwrap_or_default()
	}
}

impl<T> AcceptDefaultSealed<T> for Option<T> {}

/// notouchie
mod private {
	/// notouchie
	pub trait AcceptDefaultSealed<T> {}
}
