//! Prelude module, exporting commonly-used items

pub use crate::{
	rust_alloc,
	rust_core,
	rust_std
};

pub use rust_std::{
	assert,
	assert_eq,
	assert_ne,
	debug_assert,
	debug_assert_eq,
	debug_assert_ne,
	print,
	println,
	eprint,
	eprintln,
	cfg,
	dbg,
	unreachable,
	concat,
	compile_error,
	file,
	line,
	column,
	format,
	format_args,
	stringify,
	vec
};
pub use rust_std::borrow::{
	Cow,
	ToOwned
};
pub use rust_std::clone::Clone;
pub use rust_std::cmp::{
	PartialEq,
	Eq,
	PartialOrd,
	Ord
};
pub use rust_std::convert::{
	From,
	Into,
	TryFrom,
	TryInto,
	AsMut,
	AsRef,
	identity
};
pub use rust_std::fmt::{
	Debug,
	Display
};
pub use rust_std::future::{
	Future,
	IntoFuture
};
pub use rust_std::iter::{
	Iterator,
	FromIterator,
	IntoIterator,
	DoubleEndedIterator,
	ExactSizeIterator,
	Extend
};
pub use rust_std::marker::{
	Send,
	Sync,
	Sized,
	Unpin,
	PhantomData,
	PhantomPinned
};
pub use rust_std::mem::{
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
pub use rust_std::ops::{
	Drop,
	Fn,
	FnMut,
	FnOnce
};
pub use rust_std::option::{
	Option,
	Option::Some,
	Option::None
};
pub use rust_std::result::{
	Result,
	Result::Ok,
	Result::Err
};
pub use rust_std::string::{
	String,
	ToString
};
pub use rust_std::vec::Vec;

#[macro_export]
macro_rules! panic {
	($($stuff:tt)*) => {
		$crate::rust_std::panic!($($stuff)*)
	}
}
pub use panic;
