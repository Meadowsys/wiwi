use super::{
	*,
	array::*,
	error::*,
	float::*,
	integer::*,
	marker::*,
	none::*,
	object::*,
	string::*
};
use ::hashbrown::HashMap;
use ::ordered_float::OrderedFloat;
use std::borrow::Borrow;
use ::std::{ borrow::Cow, fmt, hint };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value<'h> {
	None,
	UnsignedInt(u128),
	SignedInt(i128),
	Float(OrderedFloat<f64>),
	Bool(bool),
	String(Cow<'h, str>),
	ArrayBorrowed(&'h [Value<'h>]),
	ArrayOwned(Vec<Value<'h>>),
	Object(HashMap<Key<'h>, Value<'h>>),
	// Bytes(Vec<u8>),
	// HomogenousArray(HomogenousArray),
}

pub type ValueOwned = Value<'static>;

impl<'h> Serialise for Value<'h> {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B, options: &Options) {
		use Value::*;
		match self {
			None => { serialise_none(output) }
			UnsignedInt(n) => { n.serialise(output, options) }
			SignedInt(n) => { n.serialise(output, options) }
			Float(f) => { f.0.serialise(output, options) }
			Bool(b) => { b.serialise(output, options) }
			String(s) => { s.serialise(output, options) }
			ArrayBorrowed(a) => { serialise_array(a, output, options) }
			ArrayOwned(a) => { serialise_array(a, output, options) }
			Object(m) => { serialise_hashbrown_into_object(m, output, options) }
		}
	}
}

impl<'h> Deserialise<'h> for Value<'h> {
	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self> {
		use Value::*;
		Ok(match input.read_byte()? {
			marker if marker_is_valid_none(marker) => { None }

			marker if marker_is_valid_u128(marker) => unsafe {
				UnsignedInt(deserialise_rest_of_u128(marker, input)?)
			}

			marker if marker_is_valid_i128(marker) => unsafe {
				SignedInt(deserialise_rest_of_i128(marker, input)?)
			}

			marker if marker_is_valid_f32(marker) => {
				Float(OrderedFloat(deserialise_rest_of_f32(input)? as _))
			}

			marker if marker_is_valid_f64(marker) => {
				Float(OrderedFloat(deserialise_rest_of_f64(input)?))
			}

			MARKER_BOOL_TRUE => { Bool(true) }
			MARKER_BOOL_FALSE => { Bool(false) }

			MARKER_STRING_8 => {
				String(Cow::Borrowed(deserialise_rest_of_str(MarkerType::M8, input)?))
			}
			MARKER_STRING_16 => {
				String(Cow::Borrowed(deserialise_rest_of_str(MarkerType::M16, input)?))
			}
			MARKER_STRING_XL => {
				String(Cow::Borrowed(deserialise_rest_of_str(MarkerType::MXL, input)?))
			}

			MARKER_ARRAY_8 => {
				ArrayOwned(deserialise_rest_of_array(MarkerType::M8, input)?)
			}
			MARKER_ARRAY_16 => {
				ArrayOwned(deserialise_rest_of_array(MarkerType::M16, input)?)
			}
			MARKER_ARRAY_XL => {
				ArrayOwned(deserialise_rest_of_array(MarkerType::MXL, input)?)
			}

			MARKER_OBJECT_8 => {
				Object(deserialise_rest_of_object_into_hashbrown(MarkerType::M8, input)?)
			}
			MARKER_OBJECT_16 => {
				Object(deserialise_rest_of_object_into_hashbrown(MarkerType::M16, input)?)
			}
			MARKER_OBJECT_XL => {
				Object(deserialise_rest_of_object_into_hashbrown(MarkerType::MXL, input)?)
			}

			_ => { return err("invalid bytes (unknown marker)") }
		})
	}
}

// TODO: I dunno if I like this (how to handle strings and arrays?)
// impl<'h> fmt::Display for Value<'h> {
// 	fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		use Value::*;
// 		match self {
// 			None => { output.write_str("None") }
// 			Bool(b) => { write!(output, "{b}") }
// 			SignedInt(n) => { write!(output, "{i}") }
// 			UnsignedInt(n) => { write!(output, "{i}") }
// 			Float(f) => { write!(output, "{}", f.0) }
// 			Array(a) => { a.iter().try_for_each(|v| write!(output, "{v}")) }
// 			String(s) => { output.write_str(s) }
// 		}
// 	}
// }

// impl<'h> Value<'h> {
// 	pub fn ensure_owned(self) -> ValueOwned {
// 		use Value::*;

// 		match self {
// 			None => { None }
// 			Bool(b) => { Bool(b) }
// 			SignedInt(n) => { SignedInt(n) }
// 			UnsignedInt(n) => { UnsignedInt(n) }
// 			Float(f) => { Float(f) }
// 			ArrayBorrowed(a) => { ArrayOwned(a.iter().cloned().map(Value::ensure_owned).collect()) }
// 			ArrayOwned(a) => { ArrayOwned(a.into_iter().map(Value::ensure_owned).collect()) }
// 			StringBorrowed(s) => { StringOwned(s.into()) }
// 			StringOwned(s) => { StringOwned(s) }
// 		}
// 	}

// 	// pub fn unwrap_bool(self) -> bool {
// 	// 	match self {
// 	// 		Value::Bool(b) => { b }
// 	// 		_ => { panic!("attempted to call unwrap_bool; value was not a bool") }
// 	// 	}
// 	// }

// 	// pub fn unwrap_u8(self) -> u8 {
// 	// 	match self {
// 	// 		Value::UnsignedInt(n) if marker_is_valid_u8(min_marker_u128(i)) => { i }
// 	// 		Value::UnsignedInt(_) => { panic!("attempted to call unwrap_u8; value would have overflowed") }
// 	// 		_ => { panic!("attempted to call unwrap_u8; value was not an unsigned u8") }
// 	// 	}
// 	// }
// }

/// Values that are allowed to be used as the key for an object
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key<'h> {
	None,
	UnsignedInt(u128),
	SignedInt(i128),
	Float(OrderedFloat<f64>),
	Bool(bool),
	String(Cow<'h, str>)
}

impl<'h> Serialise for Key<'h> {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B, options: &Options) {
		use Key::*;
		match self {
			None => { serialise_none(output) }
			UnsignedInt(n) => { n.serialise(output, options) }
			SignedInt(n) => { n.serialise(output, options) }
			Float(f) => { f.serialise(output, options) }
			Bool(b) => { b.serialise(output, options) }
			String(s) => { s.serialise(output, options) }
		}
	}
}

impl<'h> Deserialise<'h> for Key<'h> {
	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self> {
		use Key::*;
		Ok(match input.read_byte()? {
			marker if marker_is_valid_none(marker) => { None }

			marker if marker_is_valid_u128(marker) => unsafe {
				UnsignedInt(deserialise_rest_of_u128(marker, input)?)
			}

			marker if marker_is_valid_i128(marker) => unsafe {
				SignedInt(deserialise_rest_of_i128(marker, input)?)
			}

			marker if marker_is_valid_f32(marker) => {
				Float(OrderedFloat(deserialise_rest_of_f32(input)? as _))
			}

			marker if marker_is_valid_f64(marker) => {
				Float(OrderedFloat(deserialise_rest_of_f64(input)?))
			}

			MARKER_BOOL_TRUE => { Bool(true) }
			MARKER_BOOL_FALSE => { Bool(false) }

			MARKER_STRING_8 => {
				String(Cow::Borrowed(deserialise_rest_of_str(MarkerType::M8, input)?))
			}
			MARKER_STRING_16 => {
				String(Cow::Borrowed(deserialise_rest_of_str(MarkerType::M16, input)?))
			}
			MARKER_STRING_XL => {
				String(Cow::Borrowed(deserialise_rest_of_str(MarkerType::MXL, input)?))
			}

			_ => { return err("invalid bytes (unknown marker)") }
		})
	}
}
