use crate::prelude_std::*;
use crate::rc::{ RcStr, Counter };

pub mod store;

struct Modifier<N: Name = String> {
	name: N,
	ty: ModifierType,
	value: ModifierValue
}

pub trait Name {
	fn as_str(&self) -> &str;
}

macro_rules! impl_name_via_deref {
	{ [$($generics:tt)*] $($stuff:tt)+ } => {
		impl<$($generics)*> Name for $($stuff)+ {
			#[inline]
			fn as_str(&self) -> &str { self }
		}
	}
}

impl_name_via_deref! { [] String }
impl_name_via_deref! { [] &str }
impl_name_via_deref! { [] Box<str> }
impl_name_via_deref! { [] Rc<str> }
impl_name_via_deref! { [C: Counter, M] RcStr<C, M> }

/// Type of modifier (flag or with numeric value, and associated
/// metadata if applicable)
enum ModifierType {
	/// A modifier that is either present, or not (eg. Soulbound)
	///
	/// For modifiers of this type, the modifier value is meaningless, and for
	/// the sake of being deterministic, it should be set to 0.
	Flag,

	/// A modifier that has an attached number (eg. +100% Trap Disarm, or +15.3 Mining Speed)
	NumericValue {
		/// The power of 10 the stored value needs to be divided by
		/// to get the true value
		///
		/// For example, if you have mining speed with value of 76 (76.0), and decimal
		/// shift 1, you'd need to divide the value by 10^1, or shift the decimal left
		/// by 1, to get the real value of 7.6.
		decimal_shift: u8
	}
}

/// The value of a modifier, including whether or not the modifier is legendary
pub struct ModifierValue {
	/// The highest bit stores if the modifier is legendary, and the rest store
	/// the modifier's value (if applicable)
	raw: i32
}

impl ModifierValue {
	/// Maximum storable modifier value
	const MAX: i32 = i32::MAX >> 1;

	/// Minimum storable modifier value (negative)
	const MIN: i32 = i32::MIN >> 1;

	#[inline]
	pub fn new(value: i32, is_legendary: bool) -> Self {
		Self::new_checked(value, is_legendary)
			.expect("modifier value out of bounds")
	}

	#[inline]
	pub fn new_checked(value: i32, is_legendary: bool) -> Option<Self> {
		(Self::MIN..=Self::MAX).contains(&value).then(|| {
			// SAFETY: we just checked we're within the allowed range
			unsafe { Self::new_unchecked(value, is_legendary) }
		})
	}

	/// # Safety
	///
	/// `value` must be within the range `MIN..=MAX`
	#[expect(clippy::as_conversions)]
	#[inline]
	pub unsafe fn new_unchecked(value: i32, is_legendary: bool) -> Self {
		Self { raw: value | ((is_legendary as i32) << 31) }
	}

	#[inline]
	pub fn value(&self) -> i32 {
		// first shift left one to push out the legendary bit, then
		// (arithmetic) shift right one to bring back the bit with correct sign
		(self.raw << 1) >> 1
	}

	#[inline]
	pub fn is_legendary(&self) -> bool {
		self.raw >> 31 == 1
	}
}
