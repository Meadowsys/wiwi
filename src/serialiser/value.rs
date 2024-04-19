use super::{ BufferImplRead, BufferImplWrite, Deserialise, Serialise };
use super::{ error::*, float::*, integer::*, marker::*, none::* };
use ::hashbrown::HashMap;
use ::ordered_float::OrderedFloat;

#[derive(Debug, Clone, Hash)]
pub enum Value {
	None,
	Bool(bool),
	SignedInt(i128),
	UnsignedInt(u128),
	Float(OrderedFloat<f64>),
	// String(String),
	// Bytes(Vec<u8>),
	// HomogenousArray(HomogenousArray),
	// Object(HashMap<Value, Value>)
}

impl Serialise for Value {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
		match self {
			Self::None => { serialise_none(output) }
			Self::Bool(b) => { b.serialise(output) }
			Self::SignedInt(num) => { num.serialise(output) }
			Self::UnsignedInt(num) => { num.serialise(output) }
			Self::Float(num) => { num.0.serialise(output) }
		}
	}
}

impl<'h> Deserialise<'h> for Value {
	fn deserialise<B: BufferImplRead>(input: &mut B) -> Result<Self> {
		Ok(match input.read_next_byte()? {
			marker if marker_is_valid_none(marker) => { Self::None }

			marker if marker_is_valid_u128(marker) => unsafe {
				Self::UnsignedInt(deserialise_rest_of_u128(marker, input)?)
			}

			marker if marker_is_valid_i128(marker) => unsafe {
				Self::SignedInt(deserialise_rest_of_i128(marker, input)?)
			}

			marker if marker_is_valid_f32(marker) => {
				Self::Float(OrderedFloat(deserialise_rest_of_f32(input)? as _))
			}

			marker if marker_is_valid_f64(marker) => {
				Self::Float(OrderedFloat(deserialise_rest_of_f64(input)?))
			}

			MARKER_BOOL_TRUE => { Self::Bool(true) }
			MARKER_BOOL_FALSE => { Self::Bool(false) }

			_ => { return err("invalid serialised bytes") }
		})
	}
}
