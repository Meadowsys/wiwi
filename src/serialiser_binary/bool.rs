use super::{ Deserialise, Error, Input, Output, Result, Serialise, Serialiser, use_some };
use super::error::expected;
use super::error::expected::*;
use super::marker::markers::*;

impl Serialise for bool {
	type Serialiser<'h> = BoolSerialiser;

	#[inline]
	fn build_serialiser(&self) -> BoolSerialiser {
		BoolSerialiser::new(*self)
	}
}

pub struct BoolSerialiser {
	value: bool
}

impl BoolSerialiser {
	#[inline]
	fn new(val: bool) -> Self {
		Self { value: val }
	}
}

impl<'h> Serialiser<'h> for BoolSerialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		// - false is 0xa0, true is 0xa1
		// - false casts to integer 0, true to 1

		// SAFETY: provided buffer is guaranteed by caller to have reserved
		// at least the amount returned by `calc_needed_capacity` (1)
		unsafe { buf.write_byte(self.value as u8 + 0xa0) }
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
