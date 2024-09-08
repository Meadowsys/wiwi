extern crate core;

#[doc(inline)]
pub use core::marker::Sized;

#[doc(inline)]
pub use core::mem::{
	size_of,
	size_of_val,

	align_of,
	align_of_val,

	drop,
	forget,
	replace,
	swap,
	take,

	transmute,
	transmute_copy,

	zeroed
};
