use super::{ *, error::*, marker::* };
use ::std::{ ptr, slice };

pub const LARGEST_U8: u16 = u8::MAX as u16;

pub const LARGEST_U16: u32 = u16::MAX as u32;
pub const LARGEST_U24: u32 = LARGEST_U16 << 8 | u8::MAX as u32;

pub const LARGEST_U32: u64 = u32::MAX as u64;
pub const LARGEST_U40: u64 = LARGEST_U32 << 8 | u8::MAX as u64;
pub const LARGEST_U48: u64 = LARGEST_U40 << 8 | u8::MAX as u64;
pub const LARGEST_U56: u64 = LARGEST_U48 << 8 | u8::MAX as u64;

pub const LARGEST_U64: u128 = u64::MAX as u128;
pub const LARGEST_U72: u128 = LARGEST_U64 << 8 | u8::MAX as u128;
pub const LARGEST_U80: u128 = LARGEST_U72 << 8 | u8::MAX as u128;
pub const LARGEST_U88: u128 = LARGEST_U80 << 8 | u8::MAX as u128;
pub const LARGEST_U96: u128 = LARGEST_U88 << 8 | u8::MAX as u128;
pub const LARGEST_U104: u128 = LARGEST_U96 << 8 | u8::MAX as u128;
pub const LARGEST_U112: u128 = LARGEST_U104 << 8 | u8::MAX as u128;
pub const LARGEST_U120: u128 = LARGEST_U112 << 8 | u8::MAX as u128;

#[inline]
pub fn min_marker_u8(num: u8) -> u8 {
	MARKER_U8
}

pub fn min_marker_u16(num: u16) -> u8 {
	match num {
		num @ ..=LARGEST_U8 => { min_marker_u8(num as u8) }
		num => MARKER_U16
	}
}

pub fn min_marker_u32(num: u32) -> u8 {
	match num {
		num @ ..=LARGEST_U16 => { min_marker_u16(num as u16) }
		num @ ..=LARGEST_U24 => { MARKER_U24 }
		num => { MARKER_U32 }
	}
}

pub fn min_marker_u64(num: u64) -> u8 {
	match num {
		num @ ..=LARGEST_U32 => { min_marker_u32(num as u32) }
		num @ ..=LARGEST_U40 => { MARKER_U40 }
		num @ ..=LARGEST_U48 => { MARKER_U48 }
		num @ ..=LARGEST_U56 => { MARKER_U56 }
		num => { MARKER_U64 }
	}
}

pub fn min_marker_u128(num: u128) -> u8 {
	match num {
		num @ ..=LARGEST_U64 => { min_marker_u64(num as u64) }
		num @ ..=LARGEST_U72 => { MARKER_U72 }
		num @ ..=LARGEST_U80 => { MARKER_U80 }
		num @ ..=LARGEST_U88 => { MARKER_U88 }
		num @ ..=LARGEST_U96 => { MARKER_U96 }
		num @ ..=LARGEST_U104 => { MARKER_U104 }
		num @ ..=LARGEST_U112 => { MARKER_U112 }
		num @ ..=LARGEST_U120 => { MARKER_U120 }
		num => { MARKER_U128 }
	}
}

pub const LARGEST_I8: i16 = i8::MAX as i16;
pub const SMALLEST_I8: i16 = !LARGEST_I8;

pub const LARGEST_I16: i32 = i16::MAX as i32;
pub const SMALLEST_I16: i32 = !LARGEST_I16;
pub const LARGEST_I24: i32 = LARGEST_I16 << 8 | u8::MAX as i32;
pub const SMALLEST_I24: i32 = !LARGEST_I24;

pub const LARGEST_I32: i64 = i32::MAX as i64;
pub const SMALLEST_I32: i64 = !LARGEST_I32;
pub const LARGEST_I40: i64 = LARGEST_I32 << 8 | u8::MAX as i64;
pub const SMALLEST_I40: i64 = !LARGEST_I40;
pub const LARGEST_I48: i64 = LARGEST_I40 << 8 | u8::MAX as i64;
pub const SMALLEST_I48: i64 = !LARGEST_I48;
pub const LARGEST_I56: i64 = LARGEST_I48 << 8 | u8::MAX as i64;
pub const SMALLEST_I56: i64 = !LARGEST_I56;

pub const LARGEST_I64: i128 = i64::MAX as i128;
pub const SMALLEST_I64: i128 = !LARGEST_I64;
pub const LARGEST_I72: i128 = LARGEST_I64 << 8 | u8::MAX as i128;
pub const SMALLEST_I72: i128 = !LARGEST_I72;
pub const LARGEST_I80: i128 = LARGEST_I72 << 8 | u8::MAX as i128;
pub const SMALLEST_I80: i128 = !LARGEST_I80;
pub const LARGEST_I88: i128 = LARGEST_I80 << 8 | u8::MAX as i128;
pub const SMALLEST_I88: i128 = !LARGEST_I88;
pub const LARGEST_I96: i128 = LARGEST_I88 << 8 | u8::MAX as i128;
pub const SMALLEST_I96: i128 = !LARGEST_I96;
pub const LARGEST_I104: i128 = LARGEST_I96 << 8 | u8::MAX as i128;
pub const SMALLEST_I104: i128 = !LARGEST_I104;
pub const LARGEST_I112: i128 = LARGEST_I104 << 8 | u8::MAX as i128;
pub const SMALLEST_I112: i128 = !LARGEST_I112;
pub const LARGEST_I120: i128 = LARGEST_I112 << 8 | u8::MAX as i128;
pub const SMALLEST_I120: i128 = !LARGEST_I120;

#[inline]
pub fn min_marker_i8(num: i8) -> u8 {
	MARKER_I8
}

pub fn min_marker_i16(num: i16) -> u8 {
	match num {
		num @ SMALLEST_I8..=LARGEST_I8 => { min_marker_i8(num as i8) }
		num => { MARKER_I16 }
	}
}

pub fn min_marker_i32(num: i32) -> u8 {
	match num {
		num @ SMALLEST_I16..=LARGEST_I16 => { min_marker_i16(num as i16) }
		num @ SMALLEST_I24..=LARGEST_I24 => { MARKER_I24 }
		num => { MARKER_I32 }
	}
}

pub fn min_marker_i64(num: i64) -> u8 {
	match num {
		num @ SMALLEST_I32..=LARGEST_I32 => { min_marker_i32(num as i32) }
		num @ SMALLEST_I40..=LARGEST_I40 => { MARKER_I40 }
		num @ SMALLEST_I48..=LARGEST_I48 => { MARKER_I48 }
		num @ SMALLEST_I56..=LARGEST_I56 => { MARKER_I56 }
		num => { MARKER_I64 }
	}
}

pub fn min_marker_i128(num: i128) -> u8 {
	match num {
		num @ SMALLEST_I64..=LARGEST_I64 => { min_marker_i64(num as i64) }
		num @ SMALLEST_I72..=LARGEST_I72 => { MARKER_I72 }
		num @ SMALLEST_I80..=LARGEST_I80 => { MARKER_I80 }
		num @ SMALLEST_I88..=LARGEST_I88 => { MARKER_I88 }
		num @ SMALLEST_I96..=LARGEST_I96 => { MARKER_I96 }
		num @ SMALLEST_I104..=LARGEST_I104 => { MARKER_I104 }
		num @ SMALLEST_I112..=LARGEST_I112 => { MARKER_I112 }
		num @ SMALLEST_I120..=LARGEST_I120 => { MARKER_I120 }
		num => { MARKER_I128 }
	}
}

// TODO: these is_valid functions might be able to be optimised
// I think something can be done with that special property of markers that
// we rely on elsewhere (the `marker >> 1 == amount` of bytes after thing)

pub fn marker_is_valid_u8(marker: u8) -> bool {
	marker == MARKER_U8
}

pub fn marker_is_valid_u16(marker: u8) -> bool {
	match marker {
		MARKER_U16 => { true }
		_ => { marker_is_valid_u8(marker) }
	}
}

pub fn marker_is_valid_u32(marker: u8) -> bool {
	match marker {
		MARKER_U24 => { true }
		MARKER_U32 => { true }
		_ => { marker_is_valid_u16(marker) }
	}
}

pub fn marker_is_valid_u64(marker: u8) -> bool {
	match marker {
		MARKER_U40 => { true }
		MARKER_U48 => { true }
		MARKER_U56 => { true }
		MARKER_U64 => { true }
		_ => { marker_is_valid_u32(marker) }
	}
}

pub fn marker_is_valid_u128(marker: u8) -> bool {
	match marker {
		MARKER_U72 => { true }
		MARKER_U80 => { true }
		MARKER_U88 => { true }
		MARKER_U96 => { true }
		MARKER_U104 => { true }
		MARKER_U112 => { true }
		MARKER_U120 => { true }
		MARKER_U128 => { true }
		_ => { marker_is_valid_u64(marker) }
	}
}

pub fn marker_is_valid_i8(marker: u8) -> bool {
	marker == MARKER_I8
}

pub fn marker_is_valid_i16(marker: u8) -> bool {
	match marker {
		MARKER_I16 => { true }
		_ => { marker_is_valid_i8(marker) }
	}
}

pub fn marker_is_valid_i32(marker: u8) -> bool {
	match marker {
		MARKER_I24 => { true }
		MARKER_I32 => { true }
		_ => { marker_is_valid_i16(marker) }
	}
}

pub fn marker_is_valid_i64(marker: u8) -> bool {
	match marker {
		MARKER_I40 => { true }
		MARKER_I48 => { true }
		MARKER_I56 => { true }
		MARKER_I64 => { true }
		_ => { marker_is_valid_i32(marker) }
	}
}

pub fn marker_is_valid_i128(marker: u8) -> bool {
	match marker {
		MARKER_I72 => { true }
		MARKER_I80 => { true }
		MARKER_I88 => { true }
		MARKER_I96 => { true }
		MARKER_I104 => { true }
		MARKER_I112 => { true }
		MARKER_I120 => { true }
		MARKER_I128 => { true }
		_ => { marker_is_valid_i64(marker) }
	}
}

macro_rules! num_serialise_rest_fns {
	($($num:ty: $fn_name:ident)*) => {
		$(
			/// # Safety
			///
			/// The passed in marker must be valid for the int type; otherwise,
			/// invalid memory can be read from, breaking memory safety
			pub unsafe fn $fn_name<B: BufferImplWrite>(num: $num, marker: u8, output: &mut B) {
				let bytes = <$num>::to_le_bytes(num);
				output.write_slice(slice::from_raw_parts(
					&bytes as *const u8,
					(marker >> 1) as usize
				));
			}
		)*
	}
}

num_serialise_rest_fns! {
	u8: serialise_rest_of_u8
	u16: serialise_rest_of_u16
	u32: serialise_rest_of_u32
	u64: serialise_rest_of_u64
	u128: serialise_rest_of_u128

	i8: serialise_rest_of_i8
	i16: serialise_rest_of_i16
	i32: serialise_rest_of_i32
	i64: serialise_rest_of_i64
	i128: serialise_rest_of_i128
}

macro_rules! impl_number_serialise {
	($($num:ty: $rest_fn:ident, $min_marker:ident)*) => {
		$(
			impl Serialise for $num {
				fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
					let num = *self;
					let marker = $min_marker(num);

					output.write_byte(marker);
					unsafe { $rest_fn(num, marker, output) }
				}
			}
		)*
	}
}

impl_number_serialise! {
	u8: serialise_rest_of_u8, min_marker_u8
	u16: serialise_rest_of_u16, min_marker_u16
	u32: serialise_rest_of_u32, min_marker_u32
	u64: serialise_rest_of_u64, min_marker_u64
	u128: serialise_rest_of_u128, min_marker_u128

	i8: serialise_rest_of_i8, min_marker_i8
	i16: serialise_rest_of_i16, min_marker_i16
	i32: serialise_rest_of_i32, min_marker_i32
	i64: serialise_rest_of_i64, min_marker_i64
	i128: serialise_rest_of_i128, min_marker_i128
}

macro_rules! num_deserialise_rest_fns {
	($($num:ty: $fn_name:ident, $sign_extend:expr)*) => {
		$(
			/// # Safety
			///
			/// The passed in marker must be valid for the int type; otherwise,
			/// this function can write to invalid memory.
			pub unsafe fn $fn_name<'h, B: BufferImplRead<'h>>(marker: u8, input: &mut B) -> Result<$num> {
				const NUM_BYTES: usize = <$num>::BITS as usize / 8;

				let mut bytes = [0u8; NUM_BYTES];
				let bytes_ptr = &mut bytes as *mut u8;
				let count = (marker >> 1) as usize;

				let ptr = input.read_bytes_ptr(count)?;
				ptr::copy_nonoverlapping(
					ptr,
					bytes_ptr,
					count
				);

				if $sign_extend {
					if count < NUM_BYTES {
						let sign_bit = *bytes_ptr.add(count - 1) >> 7;
						if sign_bit != 0 {
							ptr::write_bytes(
								bytes_ptr.add(count),
								u8::MAX,
								NUM_BYTES - count
							);
						}
					}
				}

				Ok(<$num>::from_le_bytes(bytes))
			}
		)*
	}
}

num_deserialise_rest_fns! {
	u8: deserialise_rest_of_u8, false
	u16: deserialise_rest_of_u16, false
	u32: deserialise_rest_of_u32, false
	u64: deserialise_rest_of_u64, false
	u128: deserialise_rest_of_u128, false

	// i8 will always be one byte only, nothing to extend
	i8: deserialise_rest_of_i8, false
	i16: deserialise_rest_of_i16, true
	i32: deserialise_rest_of_i32, true
	i64: deserialise_rest_of_i64, true
	i128: deserialise_rest_of_i128, true
}

macro_rules! impl_number_deserialise {
	($($num:ty: $rest_fn:ident, $check_marker_fn:ident)*) => {
		$(
			impl<'h> Deserialise<'h> for $num {
				fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self> {
					let marker = input.read_byte()?;

					if $check_marker_fn(marker) {
						unsafe { $rest_fn(marker, input) }
					} else {
						err(concat!("expected ", stringify!($num), "-compatible integer"))
					}
				}
			}
		)*
	}
}

impl_number_deserialise! {
	u8: deserialise_rest_of_u8, marker_is_valid_u8
	u16: deserialise_rest_of_u16, marker_is_valid_u16
	u32: deserialise_rest_of_u32, marker_is_valid_u32
	u64: deserialise_rest_of_u64, marker_is_valid_u64
	u128: deserialise_rest_of_u128, marker_is_valid_u128

	i8: deserialise_rest_of_i8, marker_is_valid_i8
	i16: deserialise_rest_of_i16, marker_is_valid_i16
	i32: deserialise_rest_of_i32, marker_is_valid_i32
	i64: deserialise_rest_of_i64, marker_is_valid_i64
	i128: deserialise_rest_of_i128, marker_is_valid_i128
}
