extern crate wiwiwiwiwi;

use crate::prelude::*;
use self::private::{ AcceptDefaultSealed, InitialisationStatusSealed };

pub use wiwiwiwiwi::builder;

pub struct Uninit {
	__private: ()
}

pub struct Init {
	__private: ()
}
/// Compile-time known value for if the current type encodes initialised or not
///
/// This can be used for example for a state that has a default variant, where
/// it doesn't matter if it has been set or not before buliding. However, we
/// cannot call `assume_init` on uninitialised values! We can therefore use this
/// value to check if something has been initialised (and the type changed to
/// reflect it), and act accordingly.
///
/// Since the value is a constant known at compile time, the optimiser is
/// very likely able to elide the check.
///
/// # Safety
///
/// Implementing this trait is a promise that your [`IS_INIT`](InitialisationStatus::IS_INIT)
/// value is actually reflective of initialisation state, assuming the types
/// have been used correctly.
pub unsafe trait InitialisationStatus: InitialisationStatusSealed {
	const IS_INIT: bool;
}

// SAFETY: `Uninit` represents being uninitialised
unsafe impl InitialisationStatus for Uninit {
	const IS_INIT: bool = false;
}

impl InitialisationStatusSealed for Uninit {}

// SAFETY: `Init` represents being initialised
unsafe impl InitialisationStatus for Init {
	const IS_INIT: bool = true;
}

impl InitialisationStatusSealed for Init {}

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
	/// notouchie
	pub trait InitialisationStatusSealed {}
}
