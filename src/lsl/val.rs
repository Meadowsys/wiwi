//! Stores values and returns references to them.

use super::*;
use hashbrown::HashMap;
use std::{ cell::RefCell, fmt, num::NonZeroU64 };

#[derive(Clone, Copy)]
pub struct Val<T> {
	pub(super) id: NonZeroU64,
	pub(super) ty: T,
	pub(super) _st: SingleThreadMarker
}

thread_local! {
	static VALUES: RefCell<HashMap<u64, String>> = {
		RefCell::new(HashMap::with_capacity(128))
	}
}

pub fn store<T: ty::Type, I: IntoVal<T>>(ty: T, val: I) -> Val<T> {
	store_untyped(ty, val.into_value())
}

pub fn store_untyped<T: ty::Type>(ty: T, val: String) -> Val<T> {
	let id = util::next_id();
	let _st = PhantomData;

	VALUES.with_borrow_mut(|values| {
		values.insert(id.get(), val);
	});

	Val { id, ty, _st }
}

pub trait ValTrait {
	// re. return type: it works for what I need it for lol. Will figure out better
	// solution if/when the need arises
	fn with_value(&self, f: &mut dyn FnMut(&str) -> Result<(), fmt::Error>) -> Result<(), fmt::Error>;
}

impl<T: ty::Type> ValTrait for Val<T> {
	fn with_value(&self, f: &mut dyn FnMut(&str) -> Result<(), fmt::Error>) -> Result<(), fmt::Error> {
		VALUES.with_borrow(|vals| {
			let val = vals.get(&self.id.get())
				.expect("cannot find associated value of val reference");
			f(val)
		})
	}
}

pub trait IntoVal<T> {
	fn into_value(self) -> String;
}

// float
// integer

macro_rules! impl_int {
	($($ty:ty)*) => {
		$(
			impl IntoVal<ty::Integer> for $ty {
				fn into_value(self) -> String {
					format!("{self}")
				}
			}
		)*
	}
}

impl_int! {
	u8 u16 u32
	i8 i16 i32 i64
}

// key
// list
// rotation
// string

impl IntoVal<ty::String> for &str {
	fn into_value(self) -> String {
		format!("{self:?}")
	}
}

impl IntoVal<ty::String> for String {
	fn into_value(self) -> String {
		format!("{self:?}")
	}
}

// vector
// bool
// quaternion
