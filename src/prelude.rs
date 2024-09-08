//! Prelude module, exporting commonly-used items

pub use crate::clone::{
	Clone,
	Copy
};
pub use crate::convert::{
	Infallible,
	From,
	Into,
	TryFrom,
	TryInto,
	identity
};
pub use crate::function::{
	Fn,
	FnMut,
	FnOnce
};
pub use crate::macros::{
	assert,
	assert_eq,
	assert_ne,
	debug_assert,
	debug_assert_eq,
	debug_assert_ne
};
pub use crate::memory::{
	Sized,
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
pub use crate::option::{
	Option,
	Option::Some,
	Option::None
};
pub use crate::phantom::{
	PhantomData,
	PhantomPinned
};
pub use crate::result::{
	Result,
	Result::Ok,
	Result::Err
};
