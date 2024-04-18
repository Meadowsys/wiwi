use ::hashbrown::HashMap;
use super::{ BufferImplRead, BufferImplWrite, Deserialise, Serialise };
use super::{ error::*, marker::*, number::* };

#[derive(Debug, Clone, Hash)]
pub enum Value {
	None,
	// Bool(bool),
	// SignedInt(i128),
	// UnsignedInt(u128),
	// Float(f64),
	// String(String),
	// Bytes(Vec<u8>),
	// HomogenousArray(HomogenousArray),
	// Object(HashMap<Value, Value>)
}

impl Serialise for Value {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
		match self {
			Self::None => { output.write_byte(MARKER_NONE) }
// 			Self::Bool(b) => { b.serialise(output) }
// 			Self::UnsignedInt(num) => { num.serialise(output) }
// 			Self::SignedInt(num) => { num.serialise(output) }
		}
	}
}

impl<'h> Deserialise<'h> for Value {
	fn deserialise<B: BufferImplRead>(input: &mut B) -> Result<Self> {
		Ok(match input.read_next_byte()? {
			MARKER_NONE => { Self::None }
// 			marker @ MARKER_U8
// 			| marker @ MARKER_U16
// 			| marker @ MARKER_U24
// 			| marker @ MARKER_U32
// 			| marker @ MARKER_U40
// 			| marker @ MARKER_U48
// 			| marker @ MARKER_U56
// 			| marker @ MARKER_U64
// 			| marker @ MARKER_U72
// 			| marker @ MARKER_U80
// 			| marker @ MARKER_U88
// 			| marker @ MARKER_U96
// 			| marker @ MARKER_U104
// 			| marker @ MARKER_U112
// 			| marker @ MARKER_U120
// 			| marker @ MARKER_U128
// 			=> unsafe { Self::UnsignedInt(deserialise_rest_of_u128(marker, input)?) }

// 			marker @ MARKER_I8
// 			| marker @ MARKER_I16
// 			| marker @ MARKER_I24
// 			| marker @ MARKER_I32
// 			| marker @ MARKER_I40
// 			| marker @ MARKER_I48
// 			| marker @ MARKER_I56
// 			| marker @ MARKER_I64
// 			| marker @ MARKER_I72
// 			| marker @ MARKER_I80
// 			| marker @ MARKER_I88
// 			| marker @ MARKER_I96
// 			| marker @ MARKER_I104
// 			| marker @ MARKER_I112
// 			| marker @ MARKER_I120
// 			| marker @ MARKER_I128
// 			=> unsafe { Self::SignedInt(deserialise_rest_of_i128(marker, input)?) }

// 			MARKER_BOOL_TRUE => { Self::Bool(true) }
// 			MARKER_BOOL_FALSE => { Self::Bool(false) }

			_ => { return err("invalid serialised bytes") }
		})
	}
}
