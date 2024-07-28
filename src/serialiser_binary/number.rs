use crate::num_traits::*;
use super::{ Deserialise, Error, Input, Output, Result, Serialise, Serialiser, use_some };
use super::error::expected;
use super::error::expected::*;
use super::error::found::*;
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

#[inline]
fn next_bytes_from_le<'h, T, I: Input<'h>, const READ_BYTES: usize, const OUT_BYTES: usize>(
	buf: &mut I,
	f: fn([u8; OUT_BYTES]) -> T,
	f_extend: unsafe fn([u8; READ_BYTES]) -> [u8; OUT_BYTES]
) -> Option<T> {
	unsafe { Some(f(f_extend(*use_some!(buf.read_bytes_const::<READ_BYTES>())))) }
}
