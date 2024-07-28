use crate::num_traits::*;
use super::{ Deserialise, Error, ErrorFound, Input, Output, Result, Serialise, Serialiser, use_ok };
use super::error::expected;
use super::error::expected::*;
use super::error::found::*;
use super::marker::{ signed_smallint_range, unsigned_smallint_range };
use super::marker::markers::*;
use std::{ hint, ptr };
use std::mem::MaybeUninit;

impl Serialise for u8 {
	type Serialiser<'h> = U8Serialiser;

	#[inline]
	fn build_serialiser(&self) -> U8Serialiser {
		U8Serialiser::new(*self)
	}
}

pub struct U8Serialiser {
	byte: u8,
	needs_marker: bool
}

impl U8Serialiser {
	#[inline]
	fn new(val: u8) -> Self {
		Self {
			byte: val,
			needs_marker: val > MARKER_SMALLINT_RANGE_END
		}
	}
}

impl<'h> Serialiser<'h> for U8Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.needs_marker as usize + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.needs_marker { buf.write_byte(MARKER_U8) }
		buf.write_byte(self.byte)
	}
}

gen_int_deserialise! {
	u8
	unsigned_smallint_range
	DESC_EXPECTED_U8
	val => val
}

impl Serialise for u16 {
	type Serialiser<'h> = U16Serialiser;

	#[inline]
	fn build_serialiser(&self) -> U16Serialiser {
		U16Serialiser::new(*self)
	}
}

pub struct U16Serialiser {
	le_bytes: [u8; 2],
	byte_count: u8
}

impl U16Serialiser {
	#[inline]
	fn new(val: u16) -> Self {
		let le_bytes = val.to_le_bytes();
		let byte_count = if val <= MARKER_SMALLINT_RANGE_END.into_u16() {
			0
		} else {
			unsafe { get_byte_count_unsigned_le(le_bytes) }
		};

		Self { le_bytes, byte_count }
	}
}

impl<'h> Serialiser<'h> for U16Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.byte_count > 2 { hint::unreachable_unchecked() }

		if self.byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			let marker = ((self.byte_count - 1) << 1) | 0x80;
			buf.write_byte(marker);
			buf.write_bytes(&self.le_bytes[..self.byte_count.into_usize()]);
		}
	}
}

gen_int_deserialise! {
	u16
	unsigned_smallint_range
	DESC_EXPECTED_U16
	val => val.into_u16()
}

impl Serialise for u32 {
	type Serialiser<'h> = U32Serialiser;

	#[inline]
	fn build_serialiser(&self) -> U32Serialiser {
		U32Serialiser::new(*self)
	}
}

pub struct U32Serialiser {
	le_bytes: [u8; 4],
	byte_count: u8
}

impl U32Serialiser {
	#[inline]
	fn new(val: u32) -> Self {
		let le_bytes = val.to_le_bytes();
		let byte_count = if val <= MARKER_SMALLINT_RANGE_END.into_u32() {
			0
		} else {
			unsafe { get_byte_count_unsigned_le(le_bytes) }
		};

		Self { le_bytes, byte_count }
	}
}

impl<'h> Serialiser<'h> for U32Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.byte_count > 4 { hint::unreachable_unchecked() }

		if self.byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			let marker = ((self.byte_count - 1) << 1) | 0x80;
			buf.write_byte(marker);
			buf.write_bytes(&self.le_bytes[..self.byte_count.into_usize()]);
		}
	}
}

gen_int_deserialise! {
	u32
	unsigned_smallint_range
	DESC_EXPECTED_U32
	val => val.into_u32()
}

impl Serialise for u64 {
	type Serialiser<'h> = U64Serialiser;

	#[inline]
	fn build_serialiser(&self) -> U64Serialiser {
		U64Serialiser::new(*self)
	}
}

pub struct U64Serialiser {
	le_bytes: [u8; 8],
	byte_count: u8
}

impl U64Serialiser {
	#[inline]
	fn new(val: u64) -> Self {
		let le_bytes = val.to_le_bytes();
		let byte_count = if val <= MARKER_SMALLINT_RANGE_END.into_u64() {
			0
		} else {
			unsafe { get_byte_count_unsigned_le(le_bytes) }
		};

		Self { le_bytes, byte_count }
	}
}

impl<'h> Serialiser<'h> for U64Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.byte_count > 8 { hint::unreachable_unchecked() }

		if self.byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			let marker = ((self.byte_count - 1) << 1) | 0x80;
			buf.write_byte(marker);
			buf.write_bytes(&self.le_bytes[..self.byte_count.into_usize()]);
		}
	}
}

gen_int_deserialise! {
	u64
	unsigned_smallint_range
	DESC_EXPECTED_U64
	val => val.into_u64()
}

impl Serialise for u128 {
	type Serialiser<'h> = U128Serialiser;

	#[inline]
	fn build_serialiser(&self) -> U128Serialiser {
		U128Serialiser::new(*self)
	}
}

pub struct U128Serialiser {
	le_bytes: [u8; 16],
	byte_count: u8
}

impl U128Serialiser {
	#[inline]
	fn new(val: u128) -> Self {
		let le_bytes = val.to_le_bytes();
		let byte_count = if val <= MARKER_SMALLINT_RANGE_END.into_u128() {
			0
		} else {
			unsafe { get_byte_count_unsigned_le(le_bytes) }
		};

		Self { le_bytes, byte_count }
	}
}

impl<'h> Serialiser<'h> for U128Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.byte_count > 16 { hint::unreachable_unchecked() }

		if self.byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			let marker = ((self.byte_count - 1) << 1) | 0x80;
			buf.write_byte(marker);
			buf.write_bytes(&self.le_bytes[..self.byte_count.into_usize()]);
		}
	}
}

gen_int_deserialise! {
	u128
	unsigned_smallint_range
	DESC_EXPECTED_U128
	val => val.into_u128()
}

impl Serialise for usize {
	type Serialiser<'h> = USizeSerialiser;

	#[inline]
	fn build_serialiser(&self) -> USizeSerialiser {
		USizeSerialiser::new(*self)
	}
}

#[cfg(target_pointer_width = "64")]
pub struct USizeSerialiser {
	inner: U64Serialiser
}

#[cfg(target_pointer_width = "32")]
pub struct USizeSerialiser {
	inner: U32Serialiser
}

#[cfg(target_pointer_width = "16")]
pub struct USizeSerialiser {
	inner: U16Serialiser
}

impl USizeSerialiser {
	#[inline]
	fn new(val: usize) -> Self {
		#[cfg(target_pointer_width = "64")]
		let inner = U64Serialiser::new(val.into_u64());

		#[cfg(target_pointer_width = "32")]
		let inner = U32Serialiser::new(val.into_u32());

		#[cfg(target_pointer_width = "16")]
		let inner = U16Serialiser::new(val.into_u16());

		Self { inner }
	}
}

impl<'h> Serialiser<'h> for USizeSerialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.inner.needed_capacity()
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		self.inner.serialise(buf)
	}
}

impl<'h> Deserialise<'h> for usize {
	type Error = Error;

	#[inline]
	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<usize> {
		#[cfg(target_pointer_width = "64")]
		let val = u64::deserialise(buf);
		#[cfg(target_pointer_width = "32")]
		let val = u32::deserialise(buf);
		#[cfg(target_pointer_width = "16")]
		let val = u16::deserialise(buf);

		Ok(use_ok!(val, val => val.into_usize()))
	}
}

impl Serialise for i8 {
	type Serialiser<'h> = I8Serialiser;

	#[inline]
	fn build_serialiser(&self) -> I8Serialiser {
		I8Serialiser::new(*self)
	}
}

pub struct I8Serialiser {
	byte: u8,
	needs_marker: bool
}

impl I8Serialiser {
	#[inline]
	fn new(val: i8) -> Self {
		Self {
			byte: val.into_u8_lossy(),
			needs_marker: {
				let lower = val < MARKER_SMALLINT_NEGATIVE_RANGE_START.into_i8_lossy();
				let upper = val > MARKER_SMALLINT_RANGE_END.into_i8_lossy();
				lower && upper
			}
		}
	}
}

impl<'h> Serialiser<'h> for I8Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.needs_marker as usize + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.needs_marker { buf.write_byte(MARKER_I8) }
		buf.write_byte(self.byte)
	}
}

gen_int_deserialise! {
	i8
	signed_smallint_range
	DESC_EXPECTED_I8
	val => val.into_i8_lossy()
}

impl Serialise for i16 {
	type Serialiser<'h> = I16Serialiser;

	#[inline]
	fn build_serialiser(&self) -> I16Serialiser {
		I16Serialiser::new(*self)
	}
}

pub struct I16Serialiser {
	le_bytes: [u8; 2],
	byte_count: u8
}

impl I16Serialiser {
	#[inline]
	fn new(val: i16) -> Self {
		let le_bytes = val.to_le_bytes();
		let byte_count = {
			let lower = val >= MARKER_SMALLINT_NEGATIVE_RANGE_START.into_i8_lossy().into_i16();
			let upper = val <= MARKER_SMALLINT_RANGE_END.into_i16();

			if lower && upper {
				0
			} else {
				unsafe { get_byte_count_signed_le(le_bytes) }
			}
		};

		Self { le_bytes, byte_count }
	}
}

impl<'h> Serialiser<'h> for I16Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.byte_count > 2 { hint::unreachable_unchecked() }

		if self.byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			let marker = ((self.byte_count - 1) << 1) | 0x81;
			buf.write_byte(marker);
			buf.write_bytes(&self.le_bytes[..self.byte_count.into_usize()]);
		}
	}
}

gen_int_deserialise! {
	i16
	signed_smallint_range
	DESC_EXPECTED_I16
	val => val.into_i8_lossy().into_i16()
}

impl Serialise for i32 {
	type Serialiser<'h> = I32Serialiser;

	#[inline]
	fn build_serialiser(&self) -> I32Serialiser {
		I32Serialiser::new(*self)
	}
}

pub struct I32Serialiser {
	le_bytes: [u8; 4],
	byte_count: u8
}

impl I32Serialiser {
	#[inline]
	fn new(val: i32) -> Self {
		let le_bytes = val.to_le_bytes();
		let byte_count = {
			let lower = val >= MARKER_SMALLINT_NEGATIVE_RANGE_START.into_i8_lossy().into_i32();
			let upper = val <= MARKER_SMALLINT_RANGE_END.into_i32();

			if lower && upper {
				0
			} else {
				unsafe { get_byte_count_signed_le(le_bytes) }
			}
		};
		Self { le_bytes, byte_count }
	}
}

impl<'h> Serialiser<'h> for I32Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.byte_count > 4 { hint::unreachable_unchecked() }

		if self.byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			let marker = ((self.byte_count - 1) << 1) | 0x81;
			buf.write_byte(marker);
			buf.write_bytes(&self.le_bytes[..self.byte_count.into_usize()]);
		}
	}
}

gen_int_deserialise! {
	i32
	signed_smallint_range
	DESC_EXPECTED_I32
	val => val.into_i8_lossy().into_i32()
}

impl Serialise for i64 {
	type Serialiser<'h> = I64Serialiser;

	#[inline]
	fn build_serialiser(&self) -> I64Serialiser {
		I64Serialiser::new(*self)
	}
}

pub struct I64Serialiser {
	le_bytes: [u8; 8],
	byte_count: u8
}

impl I64Serialiser {
	#[inline]
	fn new(val: i64) -> Self {
		let le_bytes = val.to_le_bytes();
		let byte_count = {
			let lower = val >= MARKER_SMALLINT_NEGATIVE_RANGE_START.into_i8_lossy().into_i64();
			let upper = val <= MARKER_SMALLINT_RANGE_END.into_i64();

			if lower && upper {
				0
			} else {
				unsafe { get_byte_count_signed_le(le_bytes) }
			}
		};

		Self { le_bytes, byte_count }
	}
}

impl<'h> Serialiser<'h> for I64Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.byte_count > 8 { hint::unreachable_unchecked() }

		if self.byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			let marker = ((self.byte_count - 1) << 1) | 0x81;
			buf.write_byte(marker);
			buf.write_bytes(&self.le_bytes[..self.byte_count.into_usize()]);
		}
	}
}

gen_int_deserialise! {
	i64
	signed_smallint_range
	DESC_EXPECTED_I64
	val => val.into_i8_lossy().into_i64()
}

impl Serialise for i128 {
	type Serialiser<'h> = I128Serialiser;

	#[inline]
	fn build_serialiser(&self) -> I128Serialiser {
		I128Serialiser::new(*self)
	}
}

pub struct I128Serialiser {
	le_bytes: [u8; 16],
	byte_count: u8
}

impl I128Serialiser {
	#[inline]
	fn new(val: i128) -> Self {
		let le_bytes = val.to_le_bytes();
		let byte_count = {
			let lower = val >= MARKER_SMALLINT_NEGATIVE_RANGE_START.into_i8_lossy().into_i128();
			let upper = val <= MARKER_SMALLINT_RANGE_END.into_i128();

			if lower && upper {
				0
			} else {
				unsafe { get_byte_count_signed_le(le_bytes) }
			}
		};

		Self { byte_count, le_bytes }
	}
}

impl<'h> Serialiser<'h> for I128Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.byte_count > 16 { hint::unreachable_unchecked() }

		if self.byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			let marker = ((self.byte_count - 1) << 1) | 0x81;
			buf.write_byte(marker);
			buf.write_bytes(&self.le_bytes[..self.byte_count.into_usize()]);
		}
	}
}

gen_int_deserialise! {
	i128
	signed_smallint_range
	DESC_EXPECTED_I128
	val => val.into_i8_lossy().into_i128()
}

impl Serialise for isize {
	type Serialiser<'h> = ISizeSerialiser;

	#[inline]
	fn build_serialiser(&self) -> ISizeSerialiser {
		ISizeSerialiser::new(*self)
	}
}

#[cfg(target_pointer_width = "64")]
pub struct ISizeSerialiser {
	inner: I64Serialiser
}

#[cfg(target_pointer_width = "32")]
pub struct ISizeSerialiser {
	inner: I32Serialiser
}

#[cfg(target_pointer_width = "16")]
pub struct ISizeSerialiser {
	inner: I16Serialiser
}

impl ISizeSerialiser {
	#[inline]
	fn new(val: isize) -> Self {
		#[cfg(target_pointer_width = "64")]
		let inner = I64Serialiser::new(val.into_i64());

		#[cfg(target_pointer_width = "32")]
		let inner = I32Serialiser::new(val.into_i32());

		#[cfg(target_pointer_width = "16")]
		let inner = I16Serialiser::new(val.into_i16());

		Self { inner }
	}
}

impl<'h> Serialiser<'h> for ISizeSerialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		self.inner.needed_capacity()
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		self.inner.serialise(buf)
	}
}

impl<'h> Deserialise<'h> for isize {
	type Error = Error;

	#[inline]
	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<isize> {
		#[cfg(target_pointer_width = "64")]
		let val = i64::deserialise(buf);
		#[cfg(target_pointer_width = "32")]
		let val = i32::deserialise(buf);
		#[cfg(target_pointer_width = "16")]
		let val = i16::deserialise(buf);

		Ok(use_ok!(val, val => val.into_isize()))
	}
}

impl Serialise for f32 {
	type Serialiser<'h> = F32Serialiser;

	#[inline]
	fn build_serialiser(&self) -> F32Serialiser {
		F32Serialiser::new(*self)
	}
}

pub struct F32Serialiser {
	value: f32
}

impl F32Serialiser {
	#[inline]
	fn new(value: f32) -> Self {
		Self { value }
	}
}

impl<'h> Serialiser<'h> for F32Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		5
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		buf.write_byte(MARKER_F32);
		buf.write_bytes(&self.value.to_le_bytes());
	}
}

impl<'h> Deserialise<'h> for f32 {
	type Error = Error;

	#[inline]
	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<f32> {
		let marker = use_ok!(
			buf.read_byte(),
			#err err => err.expected(DESC_EXPECTED_MARKER).wrap()
		);

		match marker {
			MARKER_F32 => {
				Ok(use_ok!(
					buf.read_bytes_const(),
					bytes => f32::from_le_bytes(*bytes),
					#err err => err.expected(DESC_EXPECTED_F32).wrap()
				))
			}
			_ => { expected(DESC_EXPECTED_F32).found_something_else().wrap() }
		}
	}
}

impl Serialise for f64 {
	type Serialiser<'h> = F64Serialiser;

	#[inline]
	fn build_serialiser(&self) -> F64Serialiser {
		F64Serialiser::new(*self)
	}
}

pub struct F64Serialiser {
	value: f64
}

impl F64Serialiser {
	#[inline]
	fn new(value: f64) -> Self {
		Self { value }
	}
}

impl<'h> Serialiser<'h> for F64Serialiser {
	#[inline]
	unsafe fn needed_capacity(&self) -> usize {
		9
	}

	#[inline]
	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		buf.write_byte(MARKER_F64);
		buf.write_bytes(&self.value.to_le_bytes());
	}
}

impl<'h> Deserialise<'h> for f64 {
	type Error = Error;

	#[inline]
	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<f64> {
		let marker = use_ok!(
			buf.read_byte(),
			#err err => err.expected(DESC_EXPECTED_MARKER).wrap()
		);

		match marker {
			MARKER_F32 => {
				Ok(use_ok!(
					buf.read_bytes_const(),
					bytes => f32::from_le_bytes(*bytes).into_f64(),
					#err err => err.expected(DESC_EXPECTED_F32).wrap()
				))
			}
			MARKER_F64 => {
				Ok(use_ok!(
					buf.read_bytes_const(),
					bytes => f64::from_le_bytes(*bytes),
					#err err => err.expected(DESC_EXPECTED_F64).wrap()
				))
			}
			_ => { expected(DESC_EXPECTED_F64).found_something_else().wrap() }
		}
	}
}

#[inline]
unsafe fn get_byte_count_unsigned_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	let ptr = bytes.as_ptr();

	for i in (1..BYTES).rev() {
		// simple! just return the first byte (including the byte) where
		// its not all 0s. Iter stops after offset 1 because we'll always
		// return at least 1 byte, so no point in checking last byte.

		// SAFETY: `ptr` is to an array of size `BYTES` and `i` from loop is
		// within `0..BYTES`, so this offset is in bounds and valid for reads
		let ptr = unsafe { ptr.add(i) };

		// SAFETY: as established above `ptr` is valid for reads
		let byte = unsafe { *ptr };
		if byte != 0 { return (i + 1) as _ }
	}

	// everything is empty? just return the minimum
	1
}

#[inline]
unsafe fn get_byte_count_signed_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	debug_assert!(BYTES > 0);

	let ptr = bytes.as_ptr();

	let sign_bit = {
		// SAFETY: `ptr` is to an array of size `BYTES`, where `BYTES` is gt 0,
		// so offsetting ptr by `BYTES - 1` is in bounds and valid for reads
		let ptr = unsafe { ptr.add(BYTES - 1) };

		// SAFETY: `ptr` is valid for reads as established above
		let byte = unsafe { *ptr };

		byte >> 7
	};

	// byte that has empty data and can (probably) be safely discarded.
	// if negative, all bits 1, if positive, all bits 0
	let empty_byte = if sign_bit == 0 { 0 } else { u8::MAX };

	for i in (0..BYTES).rev() {
		let byte = {
			// SAFETY: `ptr` is to an array of size `BYTES` and `i` from loop is
			// within `0..BYTES`, so this offset is in bounds and valid for reads
			let ptr = unsafe { ptr.add(i) };

			// SAFETY: as established above this offset ptr is valid for reads
			unsafe { *ptr }
		};

		// the following could be shortened to a one liner...
		// but this absolutely sucks for readability/maintainability, so nah
		// if byte != empty_byte { return (i + 1) as u8 + (byte >> 7 != sign_bit) as u8 }

		if byte == empty_byte {
			// byte is full of 1s if negative, or full of 0s if positive
			// this byte can (probably) be safely discarded; continue
		} else if byte >> 7 == sign_bit {
			// sign bit is the same, return up to / including this byte
			// iter range is 0 to BYTES - 1 (inclusive), so this return range
			// will be 1 to BYTES (inclusive), which is correct
			return (i + 1).into_u8_lossy()
		} else {
			// sign bit is different, return this byte and one more after it.
			// if the next byte would have the wrong sign, it would have returned
			// already in the previous branch. This won't ever overflow because
			// the first byte will not have a different sign (as... itself),
			// so will never reach here
			return (i + 2).into_u8_lossy()
		}
	}

	// everything is empty? just return the minimum
	1
}

macro_rules! gen_int_deserialise {
	{
		$int:ident
		$smallint_macro:ident
		$expected:ident
		$smallint:ident => $smallint_cast:expr
	} => {
		impl<'h> Deserialise<'h> for $int {
			type Error = Error;

			fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<$int> {
				let marker = use_ok!(
					buf.read_byte(),
					#err err => err.expected(DESC_EXPECTED_MARKER).wrap()
				);

				match marker {
					$smallint @ $smallint_macro!() => { Ok($smallint_cast) }

					MARKER_U8 => { int_try_into(read_u8(buf), $expected) }
					MARKER_I8 => { int_try_into(read_i8(buf), $expected) }

					MARKER_U16 => { int_try_into(read_u16(buf), $expected) }
					MARKER_I16 => { int_try_into(read_i16(buf), $expected) }

					MARKER_U24 => { int_try_into(read_u24_to_u32(buf), $expected) }
					MARKER_I24 => { int_try_into(read_i24_to_i32(buf), $expected) }

					MARKER_U32 => { int_try_into(read_u32(buf), $expected) }
					MARKER_I32 => { int_try_into(read_i32(buf), $expected) }

					MARKER_U40 => { int_try_into(read_u40_to_u64(buf), $expected) }
					MARKER_I40 => { int_try_into(read_i40_to_i64(buf), $expected) }

					MARKER_U48 => { int_try_into(read_u48_to_u64(buf), $expected) }
					MARKER_I48 => { int_try_into(read_i48_to_i64(buf), $expected) }

					MARKER_U56 => { int_try_into(read_u56_to_u64(buf), $expected) }
					MARKER_I56 => { int_try_into(read_i56_to_i64(buf), $expected) }

					MARKER_U64 => { int_try_into(read_u64(buf), $expected) }
					MARKER_I64 => { int_try_into(read_i64(buf), $expected) }

					MARKER_U72 => { int_try_into(read_u72_to_u128(buf), $expected) }
					MARKER_I72 => { int_try_into(read_i72_to_i128(buf), $expected) }

					MARKER_U80 => { int_try_into(read_u80_to_u128(buf), $expected) }
					MARKER_I80 => { int_try_into(read_i80_to_i128(buf), $expected) }

					MARKER_U88 => { int_try_into(read_u88_to_u128(buf), $expected) }
					MARKER_I88 => { int_try_into(read_i88_to_i128(buf), $expected) }

					MARKER_U96 => { int_try_into(read_u96_to_u128(buf), $expected) }
					MARKER_I96 => { int_try_into(read_i96_to_i128(buf), $expected) }

					MARKER_U104 => { int_try_into(read_u104_to_u128(buf), $expected) }
					MARKER_I104 => { int_try_into(read_i104_to_i128(buf), $expected) }

					MARKER_U112 => { int_try_into(read_u112_to_u128(buf), $expected) }
					MARKER_I112 => { int_try_into(read_i112_to_i128(buf), $expected) }

					MARKER_U120 => { int_try_into(read_u120_to_u128(buf), $expected) }
					MARKER_I120 => { int_try_into(read_i120_to_i128(buf), $expected) }

					MARKER_U128 => { int_try_into(read_u128(buf), $expected) }
					MARKER_I128 => { int_try_into(read_i128(buf), $expected) }

					_ => { expected($expected).found_something_else().wrap() }
				}
			}
		}
	}
}
use gen_int_deserialise;

macro_rules! read_int_fn_impl {
	{ $(
		$fn_name:ident $int:ident $expected:ident
	)* } => {
		$(
			#[inline]
			fn $fn_name<'h, I: Input<'h>>(buf: &mut I) -> Result<$int> {
				Ok(use_ok!(
					buf.read_bytes_const(),
					bytes => $int::from_le_bytes(*bytes),
					#err err => err.expected($expected).wrap()
				))
			}
		)*
	}
}

read_int_fn_impl! {
	read_u8 u8 DESC_EXPECTED_U8
	read_i8 i8 DESC_EXPECTED_I8

	read_u16 u16 DESC_EXPECTED_U16
	read_i16 i16 DESC_EXPECTED_I16

	read_u32 u32 DESC_EXPECTED_U32
	read_i32 i32 DESC_EXPECTED_I32

	read_u64 u64 DESC_EXPECTED_U64
	read_i64 i64 DESC_EXPECTED_I64

	read_u128 u128 DESC_EXPECTED_U128
	read_i128 i128 DESC_EXPECTED_I128
}

macro_rules! read_int_extending_fn_impl {
	{ $(
		$fn_name:ident $in_bytes:literal $extend_fn:ident $target_int:ident $expected:ident
	)* } => {
		$(
			#[inline]
			fn $fn_name<'h, I: Input<'h>>(buf: &mut I) -> Result<$target_int> {
				Ok(use_ok!(
					buf.read_bytes_const::<$in_bytes>(),
					bytes => $target_int::from_le_bytes(unsafe { $extend_fn(*bytes) }),
					#err err => err.expected($expected).wrap()
				))
			}
		)*
	}
}

read_int_extending_fn_impl! {
	read_u24_to_u32 3 zero_extend_array_le u32 DESC_EXPECTED_U24
	read_i24_to_i32 3 zero_extend_array_le i32 DESC_EXPECTED_I24

	read_u40_to_u64 5 zero_extend_array_le u64 DESC_EXPECTED_U40
	read_i40_to_i64 5 zero_extend_array_le i64 DESC_EXPECTED_I40

	read_u48_to_u64 6 zero_extend_array_le u64 DESC_EXPECTED_U48
	read_i48_to_i64 6 zero_extend_array_le i64 DESC_EXPECTED_I48

	read_u56_to_u64 7 zero_extend_array_le u64 DESC_EXPECTED_U56
	read_i56_to_i64 7 zero_extend_array_le i64 DESC_EXPECTED_I56

	read_u72_to_u128 9 zero_extend_array_le u128 DESC_EXPECTED_U72
	read_i72_to_i128 9 zero_extend_array_le i128 DESC_EXPECTED_I72

	read_u80_to_u128 10 zero_extend_array_le u128 DESC_EXPECTED_U80
	read_i80_to_i128 10 zero_extend_array_le i128 DESC_EXPECTED_I80

	read_u88_to_u128 11 zero_extend_array_le u128 DESC_EXPECTED_U88
	read_i88_to_i128 11 zero_extend_array_le i128 DESC_EXPECTED_I88

	read_u96_to_u128 12 zero_extend_array_le u128 DESC_EXPECTED_U96
	read_i96_to_i128 12 zero_extend_array_le i128 DESC_EXPECTED_I96

	read_u104_to_u128 13 zero_extend_array_le u128 DESC_EXPECTED_U104
	read_i104_to_i128 13 zero_extend_array_le i128 DESC_EXPECTED_I104

	read_u112_to_u128 14 zero_extend_array_le u128 DESC_EXPECTED_U112
	read_i112_to_i128 14 zero_extend_array_le i128 DESC_EXPECTED_I112

	read_u120_to_u128 15 zero_extend_array_le u128 DESC_EXPECTED_U120
	read_i120_to_i128 15 zero_extend_array_le i128 DESC_EXPECTED_I120
}

#[inline]
fn int_try_into<Into, From: TryInto<Into>>(
	from: Result<From>,
	expected_msg: &'static str
) -> Result<Into> {
	Ok(use_ok!(
		use_ok!(from).try_into(),
		#err _err => expected(expected_msg)
			.found(DESC_FOUND_OVERFLOWING_INT)
			.wrap()
	))
}

#[inline]
fn next_bytes_from_le<'h, const IN_BYTES: usize, const OUT_BYTES: usize, T, I: Input<'h>>(
	buf: &mut I,
	extend: unsafe fn([u8; IN_BYTES]) -> [u8; OUT_BYTES],
	from_le_bytes: fn([u8; OUT_BYTES]) -> T
) -> Result<T, ErrorFound> {
	debug_assert!(IN_BYTES < OUT_BYTES);
	unsafe { Ok(from_le_bytes(extend(*use_ok!(buf.read_bytes_const::<IN_BYTES>())))) }
}

struct TryNextBytesFromLeParams<'h, const IN_BYTES: usize, const EXTENDED_BYTES: usize, const OUT_BYTES: usize, T, E, I, TIntermediate> {
	buf: &'h mut I,
	extend: unsafe fn([u8; IN_BYTES]) -> [u8; EXTENDED_BYTES],
	from_le_bytes: fn([u8; EXTENDED_BYTES]) -> TIntermediate,
	try_into: fn(TIntermediate) -> Result<T, E>
}

#[inline]
fn try_next_bytes_from_le<'h, const IN_BYTES: usize, const EXTENDED_BYTES: usize, const OUT_BYTES: usize, T, E, I: Input<'h>, TIntermediate>(
	params: TryNextBytesFromLeParams<'h, IN_BYTES, EXTENDED_BYTES, OUT_BYTES, T, E, I, TIntermediate>
) -> Result<T, ErrorFound> {
	let TryNextBytesFromLeParams { buf, extend, from_le_bytes, try_into } = params;
	Ok(use_ok!(
		unsafe { try_into(from_le_bytes(extend(*use_ok!(buf.read_bytes_const::<IN_BYTES>())))) },
		#err _err => super::found_something_else().wrap()
	))
}

#[inline]
unsafe fn zero_extend_array_le<
	const IN_BYTES: usize,
	const OUT_BYTES: usize
>(bytes: [u8; IN_BYTES]) -> [u8; OUT_BYTES] {
	debug_assert!(IN_BYTES < OUT_BYTES);

	let mut out = MaybeUninit::<[u8; OUT_BYTES]>::uninit();

	ptr::copy_nonoverlapping(bytes.as_ptr(), out.as_mut_ptr().cast::<u8>(), IN_BYTES);
	ptr::write_bytes(out.as_mut_ptr().cast::<u8>().add(IN_BYTES), 0, OUT_BYTES - IN_BYTES);

	out.assume_init()
}

#[inline]
unsafe fn sign_extend_array_le<
	const IN_BYTES: usize,
	const OUT_BYTES: usize
>(bytes: [u8; IN_BYTES]) -> [u8; OUT_BYTES] {
	debug_assert!(IN_BYTES < OUT_BYTES);

	let sign = bytes[IN_BYTES - 1] >> 7;
	let fill_byte = if sign == 0 { 0 } else { u8::MAX };

	let mut out = MaybeUninit::<[u8; OUT_BYTES]>::uninit();

	ptr::copy_nonoverlapping(bytes.as_ptr(), out.as_mut_ptr().cast::<u8>(), IN_BYTES);
	ptr::write_bytes(out.as_mut_ptr().cast::<u8>().add(IN_BYTES), fill_byte, OUT_BYTES - IN_BYTES);

	out.assume_init()
}
