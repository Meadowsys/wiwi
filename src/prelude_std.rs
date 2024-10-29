//! Prelude exporting types from the Rust standard library (`std`)

pub extern crate alloc;
pub extern crate core;
pub extern crate std;

pub use std::{
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
	format,
	format_args,

	panic,
	compile_error,
	todo,
	unreachable,

	cfg,
	file,
	line,
	column,

	concat,
	dbg,
	stringify,

	array,
	env,
	str
};

pub use std::any::{ Any, TypeId, type_name, type_name_of_val };
pub use std::borrow::{ Borrow, BorrowMut, Cow, ToOwned };
pub use std::boxed::Box;
pub use std::clone::{ self, Clone };
pub use std::cmp::{ self, Eq, Ord, PartialEq, PartialOrd, Ordering };
pub use std::convert::{ AsMut, AsRef, From, Into, TryFrom, TryInto, Infallible, identity };
pub use std::default::Default;
pub use std::error::Error;
pub use std::fmt::{ self, Debug, Display };
pub use std::future::{ self, Future, IntoFuture };
pub use std::hash::{ self, Hash, Hasher };
pub use std::iter::{ self, Iterator, FromIterator, IntoIterator, DoubleEndedIterator, ExactSizeIterator, Extend };
pub use std::marker::{ self, Copy, Send, Sync, Sized, Unpin, PhantomData, PhantomPinned };
pub use std::mem::{
	self,
	ManuallyDrop,
	MaybeUninit,
	align_of,
	align_of_val,
	size_of,
	size_of_val,
	transmute,
	transmute_copy,
	drop,
	forget,
	needs_drop,
	replace,
	swap,
	take,
	zeroed
};
pub use std::ops::{ self, Drop, Fn, FnMut, FnOnce };
pub use std::option::{ self, Option, Option::Some, Option::None };
pub use std::result::{ self, Result, Result::Ok, Result::Err };
pub use std::string::{ self, String, ToString };
pub use std::vec::{ self, Vec };
