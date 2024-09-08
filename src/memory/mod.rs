extern crate core;

#[doc(inline)]
pub use core::marker::Sized;

#[doc(inline)]
pub use core::mem::{
	size_of,
	size_of_val,

	align_of,
	align_of_val,

	transmute,
	transmute_copy,

	drop,
	forget,
	replace,
	swap,
	take,

	zeroed
};
