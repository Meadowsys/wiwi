#![allow(clippy::match_overlapping_arm)]

use super::{ *, buffer::*, error::*, integer::* };
use std::ptr;

// we are just going to be responsible for serialising/deserialising XL lengths,
// nothing else (not 8 variant etc)

pub const MAX_U12: u64 = 0xf_ff;
pub const MAX_U20: u64 = 0xf_ff_ff;
pub const MAX_U28: u64 = 0xf_ff_ff_ff;
pub const MAX_U36: u64 = 0xf_ff_ff_ff_ff;
pub const MAX_U44: u64 = 0xf_ff_ff_ff_ff_ff;
pub const MAX_U52: u64 = 0xf_ff_ff_ff_ff_ff_ff;
pub const MAX_U60: u64 = 0xf_ff_ff_ff_ff_ff_ff_ff;

pub const MAX_VALID_MARKER: u8 = MARKER_U60;

pub const MARKER_U12: u8 = 0;
pub const MARKER_U20: u8 = 1;
pub const MARKER_U28: u8 = 2;
pub const MARKER_U36: u8 = 3;
pub const MARKER_U44: u8 = 4;
pub const MARKER_U52: u8 = 5;
pub const MARKER_U60: u8 = 6;
pub const MARKER_U64: u8 = 7;
// pub const MARKER_U68: u8 = 7;
// pub const MARKER_U76: u8 = 8;
// pub const MARKER_U84: u8 = 9;
// pub const MARKER_U92: u8 = 10;
// pub const MARKER_U100: u8 = 11;
// pub const MARKER_U108: u8 = 12;
// pub const MARKER_U116: u8 = 13;
// pub const MARKER_U124: u8 = 14;
// pub const MARKER_U1128: u8 = 15;

exported_match_macro!(fits_in_u12: ..=MAX_U12);
exported_match_macro!(fits_in_u20: ..=MAX_U20);
exported_match_macro!(fits_in_u28: ..=MAX_U28);
exported_match_macro!(fits_in_u36: ..=MAX_U36);
exported_match_macro!(fits_in_u44: ..=MAX_U44);
exported_match_macro!(fits_in_u52: ..=MAX_U52);
exported_match_macro!(fits_in_u60: ..=MAX_U60);
exported_match_macro!(fits_in_u64: ..=MAX_U64);

// these functions would only be called when we _know_ we need to
// deserialise one of these (like, ex. after a collection marker), so the
// deserialise-rest functions aren't needed.

pub fn serialise_len_int<B: BufferWrite>(len: usize, output: &mut B) {
	let bytes = len.to_le_bytes();
	let bytes_ptr = &bytes as *const u8;

	let marker = match len as u64 {
		fits_in_u12!() => { MARKER_U12 }
		fits_in_u20!() => { MARKER_U20 }
		fits_in_u28!() => { MARKER_U28 }
		fits_in_u36!() => { MARKER_U36 }
		fits_in_u44!() => { MARKER_U44 }
		fits_in_u52!() => { MARKER_U52 }
		fits_in_u60!() => { MARKER_U60 }
		_ => unsafe {
			output.reserve(9);
			output.with_ptr(|ptr| {
				ptr::write(ptr, MARKER_U64);
				ptr::copy_nonoverlapping(bytes_ptr, ptr.add(1), 8);
				9
			});
			return
		}
	};

	let len = marker as usize + 1;

	output.reserve(len + 1);
	unsafe {
		output.with_ptr(|ptr| {
			let last_4_bits = *bytes_ptr.add(len) << 4;
			let marker = marker | last_4_bits;

			ptr::write(ptr, marker);
			ptr::copy_nonoverlapping(bytes_ptr, ptr.add(1), len);

			len + 1
		});
	}
}

pub fn deserialise_len_int<'h, B: BufferRead<'h>>(input: &mut B) -> Result<usize> {
	macro_rules! usize_overflow_check {
		($len:ident) => {
			#[cfg(not(target_pointer_width = "64"))]
			if $len > usize::MAX as u64 {
				return err("length overflows platform word size")
			}
		}
	}

	let marker = input.read_byte()?;
	let (marker, last_4_bits) = match marker & 0b1111 {
		m @ ..=6 => { (m, marker >> 4) }
		7 => {
			let len = u64::from_le_bytes(*input.read_bytes_const()?);
			usize_overflow_check!(len);
			return Ok(len as _)
		}
		_ => { return err("expected len int") }
	};

	let mut bytes = [0u8; 8];
	let bytes_ptr = &mut bytes as *mut u8;

	let len = marker as usize + 1;
	unsafe {
		let read = input.read_bytes_ptr(len)?;
		ptr::copy_nonoverlapping(read, bytes_ptr, len);
		ptr::write(bytes_ptr.add(len), last_4_bits);
	}

	let len = u64::from_le_bytes(bytes);
	usize_overflow_check!(len);
	Ok(len as _)
}
