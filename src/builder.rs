extern crate wiwiwiwiwi;

use crate::prelude::*;
use self::private::*;

pub use wiwiwiwiwi::builder;

pub struct Uninit {
	__private: ()
}

pub trait IsUninit: IsUninitSealed {}

impl IsUninit for Uninit {}
impl IsUninitSealed for Uninit {}

pub struct Init {
	__private: ()
}

pub trait IsInit: IsInitSealed {}

impl IsInit for Init {}
impl IsInitSealed for Init {}

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
/// Implementing this trait is a promise that your
/// [`IS_INIT`](InitialisationStatus::IS_INIT)
/// value is actually reflective of initialisation state, and
/// [`InvertInitialisationStatus`](InitialisationStatus::InvertInitialisationStatus)
/// is too, assuming the types have been used correctly.
pub unsafe trait InitialisationStatus: InitialisationStatusSealed {
	const IS_INIT: bool;
	const IS_UNINIT: bool = !Self::IS_INIT;
	type InvertInitialisationStatus: InitialisationStatus;
}

// SAFETY: `Uninit` represents being uninitialised
unsafe impl InitialisationStatus for Uninit {
	const IS_INIT: bool = false;
	type InvertInitialisationStatus = Init;
}

impl InitialisationStatusSealed for Uninit {}

// SAFETY: `Init` represents being initialised
unsafe impl InitialisationStatus for Init {
	const IS_INIT: bool = true;
	type InvertInitialisationStatus = Uninit;
}

impl InitialisationStatusSealed for Init {}

pub type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;

/// notouchie
mod private {
	/// notouchie
	pub trait IsUninitSealed {}
	/// notouchie
	pub trait IsInitSealed {}
	/// notouchie
	pub trait InitialisationStatusSealed {}
}
