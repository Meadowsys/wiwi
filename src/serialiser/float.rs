use super::{ *, error::*, marker::* };

#[inline]
pub fn marker_is_valid_f32(marker: u8) -> bool {
	marker == MARKER_F32
}

#[inline]
pub fn marker_is_valid_f64(marker: u8) -> bool {
	matches!(marker, MARKER_F32 | MARKER_F64)
}

#[inline]
pub fn serialise_rest_of_f32<B: BufferImplWrite>(float: f32, output: &mut B) {
	output.write_slice(&float.to_le_bytes());
}

#[inline]
pub fn serialise_rest_of_f64<B: BufferImplWrite>(float: f64, output: &mut B) {
	output.write_slice(&float.to_le_bytes());
}

#[inline]
pub fn deserialise_rest_of_f32<'h, B: BufferImplRead<'h>>(input: &mut B) -> Result<f32> {
	Ok(f32::from_le_bytes(*input.read_bytes_const()?))
}

#[inline]
pub fn deserialise_rest_of_f64<'h, B: BufferImplRead<'h>>(input: &mut B) -> Result<f64> {
	Ok(f64::from_le_bytes(*input.read_bytes_const()?))
}

impl Serialise for f32 {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B, options: &Options) {
		output.write_byte(MARKER_F32);
		serialise_rest_of_f32(*self, output);
	}
}

impl Serialise for f64 {
	fn serialise<B: BufferImplWrite>(&self, output: &mut B, options: &Options) {
		output.write_byte(MARKER_F64);
		serialise_rest_of_f64(*self, output);
	}
}

impl<'h> Deserialise<'h> for f32 {
	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self> {
		if marker_is_valid_f32(input.read_byte()?) {
			deserialise_rest_of_f32(input)
		} else {
			err("expected f32")
		}
	}
}

impl<'h> Deserialise<'h> for f64 {
	fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self> {
		match input.read_byte()? {
			MARKER_F32 => { deserialise_rest_of_f32(input).map(|f| f as _) }
			MARKER_F64 => { deserialise_rest_of_f64(input) }
			_ => { err("expected f64-compatible float") }
		}
	}
}

// TODO: downcasting things. WIP, messy, ditched for now

// pub struct FloatDowncast {
// 	inner: FloatDowncastInner
// }
//
// /// Internally stores the bits in little endian order
// pub enum FloatDowncastInner {
// 	F64 {
// 		le: [u8; 8]
// 	},
// 	F32 {
// 		le: [u8; 4]
// 	},
// 	F16 {
// 		le: [u8; 2]
// 	}
// }
//
// impl FloatDowncast {
// 	pub fn from_f64(float: f64) -> Self {
// 		todo!()
// 	}
//
// 	pub fn from_f32(float: f32) -> Self {
// 		todo!()
// 	}
//
// 	pub fn get_marker(&self) -> u8 {
// 		match &self.inner {
// 			FloatDowncastInner::F64 { .. } => { MARKER_F64 }
// 			FloatDowncastInner::F32 { .. } => { MARKER_F32 }
// 			FloatDowncastInner::F16 { .. } => { MARKER_F16 }
// 		}
// 	}
//
// 	pub fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
// 		use FloatDowncastInner::*;
// 		match &self.inner {
// 			F64 { le } => {
// 				output.write_byte(MARKER_F64);
// 				output.write_slice(le);
// 			}
// 			F32 { le } => {
// 				output.write_byte(MARKER_F32);
// 				output.write_slice(le);
// 			}
// 			F16 { le } => {
// 				output.write_byte(MARKER_F16);
// 				output.write_slice(le);
// 			}
// 		}
// 	}
// }

// s: sign, e: exponent, f: fraction
// f16: seeeeeffffffffff (5 exponent 10 fraction)
// f32: seeeeeeeefffffffffffffffffffffff (8 exponent 23 fraction)
// f64: seeeeeeeeeeeffffffffffffffffffffffffffffffffffffffffffffffffffff (11 exponent 52 fraction)
