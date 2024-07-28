use crate::num_traits::*;
use super::{ Deserialise, Error, Input, Output, Result, Serialise, Serialiser, use_marker, use_ok, USizeSerialiser };
use super::error::expected;
use super::error::expected::*;
use super::error::found::*;
use super::marker::markers::*;
use std::borrow::Cow;
use std::str;

impl Serialise for str {
	type Serialiser<'h> = StrSerialiser<'h>;

	#[inline]
	fn build_serialiser(&self) -> StrSerialiser<'_> {
		StrSerialiser::new(self)
	}
}

pub struct StrSerialiser<'h> {
	val: &'h str,
	/// If `val.len() > u8::MAX`, this will be `Some`, containing
	/// the [`USizeSerialiser`] whose job is to serialise the length
	len_ser: Option<USizeSerialiser>
}

impl<'h> StrSerialiser<'h> {
	#[inline]
	fn new(val: &'h str) -> Self {
		let len_ser = if val.len() > u8::MAX.into_usize() {
			Some(USizeSerialiser::new(val.len()))
		} else {
			None
		};

		Self { val, len_ser }
	}
}

impl<'h> Serialiser<'h> for StrSerialiser<'h> {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		let meta = if let Some(len_ser) = self.len_ser.as_ref() {
			// marker + length serialised
			1 + len_ser.needed_capacity()
		} else {
			// marker + one byte for len
			2
		};

		meta + self.val.len()
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if let Some(len_ser) = self.len_ser.as_ref() {
			buf.write_byte(MARKER_STR_XL);
			len_ser.serialise(buf);
		} else {
			buf.write_byte(MARKER_STR_8);
			buf.write_byte(self.val.len().into_u8_lossy());
		}

		buf.write_bytes(self.val.as_bytes());
	}
}

impl<'h> Deserialise<'h> for &'h str {
	type Error = Error;

	#[inline]
	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<&'h str> {
		let marker = use_marker!(buf);

		let len = match marker {
			MARKER_STR_8 => {
				use_ok!(
					buf.read_byte(),
					#err err => err.expected(DESC_EXPECTED_STR).wrap()
				).into_usize()
			}
			MARKER_STR_XL => {
				use_ok!(
					usize::deserialise(buf),
					#err err => err.expected(DESC_EXPECTED_STR).wrap()
				)
			}
			_ => {
				return expected(DESC_EXPECTED_STR)
					.found_something_else()
					.wrap()
			}
		};

		let slice = use_ok!(
			buf.read_bytes(len),
			#err err => err.expected(DESC_EXPECTED_STR).wrap()
		);

		let str = use_ok!(
			str::from_utf8(slice),
			#err _err => expected(DESC_EXPECTED_STR)
				.found(DESC_FOUND_INVALID_UTF_8)
				.wrap()
		);

		Ok(str)
	}
}

impl<'h> Deserialise<'h> for String {
	type Error = Error;

	#[inline]
	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<String> {
		Ok(use_ok!(
			<&str>::deserialise(buf),
			str => String::from(str)
		))
	}
}

impl<'h> Deserialise<'h> for Cow<'h, str> {
	type Error = Error;

	#[inline]
	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<Cow<'h, str>> {
		Ok(use_ok!(
			<&str>::deserialise(buf),
			str => Cow::Borrowed(str)
		))
	}
}
