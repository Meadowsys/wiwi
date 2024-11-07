use crate::prelude_std::*;
use crate::rc::RcStrAtomic;
use super::ModifierValue;

/// Jewel storage (a bundle of jewels)
pub struct JewelStore {
	jewels: Vec<Jewel>,
	modifiers: Vec<RcStrAtomic<ModifierMeta>>,
}

/// A jewel from a jewel store
struct Jewel {
	level: u16,
	size: u8,
	modifiers: Modifiers
}

/// The modifiers a jewel can have
enum Modifiers {
	Chipped([Modifier; 1]),
	Flawed([Modifier; 2]),
	Flawless([Modifier; 3]),
	Perfect([Modifier; 4])
}

/// An "instance" of a jewel modifier (with a specific value, present on a jewel)
struct Modifier {
	name: RcStrAtomic<ModifierMeta>,
	value: ModifierValue
}

/// Metadata stored with the modifier type, that is true for all "instances"
/// of the modifier, no matter its concrete stat value
struct ModifierMeta {
	ty: ModifierType
}

/// Type of modifier (flag or with numeric value, and associated
/// metadata if applicable)
enum ModifierType {
	Flag,
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
