use super::{ buffer::*, core::*, error::* };
use std::ptr;

// s: sign, e: exponent, f: fraction
// f16: seeeeeffffffffff (5 exponent 10 fraction)
// f32: seeeeeeeefffffffffffffffffffffff (8 exponent 23 fraction)
// f64: seeeeeeeeeeeffffffffffffffffffffffffffffffffffffffffffffffffffff (11 exponent 52 fraction)

pub const MARKER_F16: u8 = 0xa2;
pub const MARKER_F32: u8 = 0xa3;
pub const MARKER_F64: u8 = 0xa4;
pub const MARKER_F128: u8 = 0xa5;
pub const MARKER_F256: u8 = 0xa6;

impl Serialise for f32 {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		let bytes = self.to_le_bytes();
		output.reserve(5);
		unsafe {
			output.with_ptr(|ptr| {
				ptr::write(ptr, MARKER_F32);
				ptr::copy_nonoverlapping(&bytes as *const u8, ptr.add(1), 4);
				5
			});
		}
	}
}

impl Serialise for f64 {
	fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
		let bytes = self.to_le_bytes();
		output.reserve(9);
		unsafe {
			output.with_ptr(|ptr| {
				ptr::write(ptr, MARKER_F64);
				ptr::copy_nonoverlapping(&bytes as *const u8, ptr.add(1), 8);
				9
			});
		}
	}
}

#[inline]
pub fn marker_valid_for_f32(marker: u8) -> bool {
	marker == MARKER_F32
}

#[inline]
pub fn marker_valid_for_f64(marker: u8) -> bool {
	matches!(marker, MARKER_F32 | MARKER_F64)
}

#[inline]
pub fn deserialise_rest_of_f32<'h, B: BufferRead<'h>>(input: &mut B) -> Result<f32> {
	Ok(f32::from_le_bytes(*input.read_bytes_const()?))
}

#[inline]
pub fn deserialise_rest_of_f64<'h, B: BufferRead<'h>>(input: &mut B) -> Result<f64> {
	Ok(f64::from_le_bytes(*input.read_bytes_const()?))
}

impl<'h> Deserialise<'h> for f32 {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
		match input.read_byte()? {
			MARKER_F32 => { deserialise_rest_of_f32(input) }
			_ => { err("expected f32-compatible float") }
		}
	}
}

impl<'h> Deserialise<'h> for f64 {
	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
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
