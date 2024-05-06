//! Stores values and returns references to them.

use super::*;
use hashbrown::HashMap;
use std::{ cell::RefCell, fmt, num::NonZeroU64 };

pub fn store<T: ty::Type, I: IntoVal<T>>(ty: T, val: I) -> Val<T> {
	store_untyped(ty, val.to_value())
}

pub fn store_untyped<T: ty::Type>(ty: T, val: String) -> Val<T> {
	let id = util::next_id();
	let _st = PhantomData;

	VALUES.with_borrow_mut(|values| {
		values.insert(id.get(), val);
	});

	Val { id, ty, _st }
}

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

impl<T: ty::Type> Val<T> {
	pub(in crate::lsl) fn with<R>(&self, f: impl FnOnce(&str) -> R) -> R {
		VALUES.with_borrow(|vals| {
			let val = vals.get(&self.id.get())
				.expect("invalid state: cannot find associated value of val reference");
			f(val)
		})
	}
}

pub trait ValTrait {
	// re. return type: it works for what I need it for lol. Will figure out better
	// solution if/when the need arises
	fn with(&self, f: &mut dyn FnMut(&str) -> Result<(), fmt::Error>) -> Result<(), fmt::Error>;
}

impl<T: ty::Type> ValTrait for Val<T> {
	fn with(&self, f: &mut dyn FnMut(&str) -> Result<(), fmt::Error>) -> Result<(), fmt::Error> {
		self.with(f)
	}
}

pub trait IntoVal<T> {
	fn to_value(&self) -> String;
}

impl<T: ty::Type, I> IntoVal<T> for &I
where
	I: IntoVal<T>
{
	fn to_value(&self) -> String {
		(**self).to_value()
	}
}

impl<T: ty::Type, I> IntoVal<T> for &mut I
where
	I: IntoVal<T>
{
	fn to_value(&self) -> String {
		(**self).to_value()
	}
}

macro_rules! impl_init_value {
	($($ty:ty, $target:ty, $closure:expr;)*) => {
		$(
			impl IntoVal<$target> for $ty {
				fn to_value(&self) -> String {
					#[inline(always)]
					fn call(item: &$ty, f: impl FnOnce(&$ty) -> String) -> String {
						// ...sure I guess
						f(item)
					}

					call(self, $closure)
				}
			}
		)*
	}
}

impl_init_value! {
	// float
	f32, ty::Float, |f| format!("{f}");

	// integer
	u8, ty::Integer, |i| format!("{i}");
	u16, ty::Integer, |i| format!("{i}");
	u32, ty::Integer, |i| format!("{i}");
	i8, ty::Integer, |i| format!("{i}");
	i16, ty::Integer, |i| format!("{i}");
	i32, ty::Integer, |i| format!("{i}");
	i64, ty::Integer, |i| format!("{i}");

	// key
	&str, ty::Key, |s| format!("{s:?}");
	String, ty::Key, |s| format!("{s:?}");

	// list
	// idea: another trait, implement for everything except lists?
	// and then blanket impl it, say like a slice of that trait impls
	// init value where the items impls that new trait

	// rotation

	// string
	&str, ty::String, |s| format!("{s:?}");
	String, ty::String, |s| format!("{s:?}");

	// vector
	(f32, f32, f32), ty::Vector, |(f1, f2, f3)| format!("<{f1}, {f2}, {f3}>");
	[f32; 3], ty::Vector, |[f1, f2, f3]| format!("<{f1}, {f2}, {f3}>");

	// bool
	bool, ty::Boolean, |b| format!("{}", *b as usize);

	// quaternion
}
