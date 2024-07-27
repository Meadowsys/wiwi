use super::error::{ Error, Result, expected };
use super::error::expected::*;
use super::marker::markers::*;
use super::{ Deserialise, Input, Output, Serialise, use_ok, use_some };

impl Serialise for bool {
	#[inline]
	unsafe fn calc_needed_capacity(&self) -> usize {
		1
	}

	#[inline]
	fn serialise<O: Output>(&self, buf: &mut O) {
		// - false is 0xa0, true is 0xa1
		// - true casts to integer 1

		// SAFETY: provided buffer is guaranteed by caller to have reserved
		// at least 1 more byte, as returned by `calc_needed_capacity`
		unsafe { buf.write_byte(*self as u8 + 0xa0) }
	}
}

impl<'h> Deserialise<'h> for bool {
	type Error = Error;

	#[inline]
	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<bool> {
		Ok(use_some!(
			buf.read_byte(),
			byte => if byte >> 1 == MARKER_BOOL_UPPER_BITS_COMMON {
				byte & 0b1 != 0
			} else {
				return expected(DESC_EXPECTED_BOOL)
					.found_something()
					.wrap_in_err()
			},
			none => expected(DESC_EXPECTED_BOOL)
				.found_nothing()
				.wrap_in_err()
		))
	}
}
