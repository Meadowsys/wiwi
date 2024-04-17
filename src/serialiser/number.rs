use super::{ BufferImplRead, BufferImplWrite, Deserialise, Serialise };
use super::{ error::*, marker::* };
use ::std::convert::identity;
use ::std::mem::{ MaybeUninit, transmute };
use ::std::{ ptr, slice };

const LARGEST_8: u16 = u8::MAX as u16;

const LARGEST_16: u32 = u16::MAX as u32;
const LARGEST_24: u32 = LARGEST_16 << 8 | u8::MAX as u32;

const LARGEST_32: u64 = u32::MAX as u64;
const LARGEST_40: u64 = LARGEST_32 << 8 | u8::MAX as u64;
const LARGEST_48: u64 = LARGEST_40 << 8 | u8::MAX as u64;
const LARGEST_56: u64 = LARGEST_48 << 8 | u8::MAX as u64;

const LARGEST_64: u128 = u64::MAX as u128;
const LARGEST_72: u128 = LARGEST_64 << 8 | u8::MAX as u128;
const LARGEST_80: u128 = LARGEST_72 << 8 | u8::MAX as u128;
const LARGEST_88: u128 = LARGEST_80 << 8 | u8::MAX as u128;
const LARGEST_96: u128 = LARGEST_88 << 8 | u8::MAX as u128;
const LARGEST_104: u128 = LARGEST_96 << 8 | u8::MAX as u128;
const LARGEST_112: u128 = LARGEST_104 << 8 | u8::MAX as u128;
const LARGEST_120: u128 = LARGEST_112 << 8 | u8::MAX as u128;

#[inline]
fn min_marker_u8(num: u8) -> u8 {
	MARKER_U8
}

fn min_marker_u16(num: u16) -> u8 {
	match num {
		num @ ..=LARGEST_8 => { min_marker_u8(num as u8) }
		num => MARKER_U16
	}
}

fn min_marker_u32(num: u32) -> u8 {
	match num {
		num @ ..=LARGEST_16 => { min_marker_u16(num as u16) }
		num @ ..=LARGEST_24 => { MARKER_U24 }
		num => { MARKER_U32 }
	}
}

fn min_marker_u64(num: u64) -> u8 {
	match num {
		num @ ..=LARGEST_32 => { min_marker_u32(num as u32) }
		num @ ..=LARGEST_40 => { MARKER_U40 }
		num @ ..=LARGEST_48 => { MARKER_U48 }
		num @ ..=LARGEST_56 => { MARKER_U56 }
		num => { MARKER_U64 }
	}
}

fn min_marker_u128(num: u128) -> u8 {
	match num {
		num @ ..=LARGEST_64 => { min_marker_u64(num as u64) }
		num @ ..=LARGEST_72 => { MARKER_U72 }
		num @ ..=LARGEST_80 => { MARKER_U80 }
		num @ ..=LARGEST_88 => { MARKER_U88 }
		num @ ..=LARGEST_96 => { MARKER_U96 }
		num @ ..=LARGEST_104 => { MARKER_U104 }
		num @ ..=LARGEST_112 => { MARKER_U112 }
		num @ ..=LARGEST_120 => { MARKER_U120 }
		num => { MARKER_U128 }
	}
}

#[inline]
fn min_marker_i8(num: i8) -> u8 {
	MARKER_I8
}

fn min_marker_i16(num: i16) -> u8 {
	match unsafe { transmute(num) } {
		num @ ..=LARGEST_8 => unsafe { min_marker_i8(transmute(num as u8)) }
		num => { MARKER_I16 }
	}
}

fn min_marker_i32(num: i32) -> u8 {
	match unsafe { transmute(num) } {
		num @ ..=LARGEST_16 => unsafe { min_marker_i16(transmute(num as u16)) }
		num @ ..=LARGEST_24 => { MARKER_I24 }
		num => { MARKER_I32 }
	}
}

fn min_marker_i64(num: i64) -> u8 {
	match unsafe { transmute(num) } {
		num @ ..=LARGEST_32 => unsafe { min_marker_i32(transmute(num as u32)) }
		num @ ..=LARGEST_40 => { MARKER_I40 }
		num @ ..=LARGEST_48 => { MARKER_I48 }
		num @ ..=LARGEST_56 => { MARKER_I56 }
		num => { MARKER_I64 }
	}
}

fn min_marker_i128(num: i128) -> u8 {
	match unsafe { transmute(num) } {
		num @ ..=LARGEST_64 => unsafe { min_marker_i64(transmute(num as u64)) }
		num @ ..=LARGEST_72 => { MARKER_I72 }
		num @ ..=LARGEST_80 => { MARKER_I80 }
		num @ ..=LARGEST_88 => { MARKER_I88 }
		num @ ..=LARGEST_96 => { MARKER_I96 }
		num @ ..=LARGEST_104 => { MARKER_I104 }
		num @ ..=LARGEST_112 => { MARKER_I112 }
		num @ ..=LARGEST_120 => { MARKER_I120 }
		num => { MARKER_I128 }
	}
}

macro_rules! impl_number_serialise {
	($($num:ty: $marker_fn:ident)*) => {
		$(
			impl Serialise for $num {
				fn serialise<B: BufferImplWrite>(&self, output: &mut B) {
					let num = *self;
					unsafe {
						let marker = $marker_fn(num);
						let bytes = <$num>::to_le_bytes(num);
						output.write_byte(marker);
						output.write_slice(slice::from_raw_parts(
							&bytes as *const u8,
							(marker >> 1) as usize
						));
					}
				}
			}
		)+
	}
}

impl_number_serialise! {
	u8: min_marker_u8
	u16: min_marker_u16
	u32: min_marker_u32
	u64: min_marker_u64
	u128: min_marker_u128

	i8: min_marker_i8
	i16: min_marker_i16
	i32: min_marker_i32
	i64: min_marker_i64
	i128: min_marker_i128
}

// TODO: these is_valid functions might be able to be optimised
// I think something can be done with that special property of markers that
// we rely on elsewhere (the `marker >> 1 == amount` of bytes after thing)

fn marker_is_valid_u8(marker: u8) -> bool {
	marker == MARKER_U8
}

fn marker_is_valid_u16(marker: u8) -> bool {
	match marker {
		MARKER_U16 => { true }
		_ => { marker_is_valid_u8(marker) }
	}
}

fn marker_is_valid_u32(marker: u8) -> bool {
	match marker {
		MARKER_U24 => { true }
		MARKER_U32 => { true }
		_ => { marker_is_valid_u16(marker) }
	}
}

fn marker_is_valid_u64(marker: u8) -> bool {
	match marker {
		MARKER_U40 => { true }
		MARKER_U48 => { true }
		MARKER_U56 => { true }
		MARKER_U64 => { true }
		_ => { marker_is_valid_u32(marker) }
	}
}

fn marker_is_valid_u128(marker: u8) -> bool {
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

fn marker_is_valid_i8(marker: u8) -> bool {
	marker == MARKER_I8
}

fn marker_is_valid_i16(marker: u8) -> bool {
	match marker {
		MARKER_I16 => { true }
		_ => { marker_is_valid_i8(marker) }
	}
}

fn marker_is_valid_i32(marker: u8) -> bool {
	match marker {
		MARKER_I24 => { true }
		MARKER_I32 => { true }
		_ => { marker_is_valid_i16(marker) }
	}
}

fn marker_is_valid_i64(marker: u8) -> bool {
	match marker {
		MARKER_I40 => { true }
		MARKER_I48 => { true }
		MARKER_I56 => { true }
		MARKER_I64 => { true }
		_ => { marker_is_valid_i32(marker) }
	}
}

fn marker_is_valid_i128(marker: u8) -> bool {
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
macro_rules! impl_number_deserialise {
	($($num:ty: $marker_fn:ident)*) => {
		$(
			impl<'h> Deserialise<'h> for $num {
				fn deserialise<B: BufferImplRead>(input: &mut B) -> Result<Self> {
					let marker = input.read_next_byte().err_eof()?;

					if $marker_fn(marker) {
						let mut buf = [0u8; { <$num>::BITS as usize / 8 }];
						let count = (marker >> 1) as usize;

						unsafe {
							let ptr = input
								.read_next_bytes_ptr(count)
								.err_eof()?;
							ptr::copy_nonoverlapping(
								ptr,
								&mut buf as *mut u8,
								count
							);
						}

						Ok(Self::from_le_bytes(buf))
					} else {
						err(concat!("expected ", stringify!($num), "-compatible integer"))
					}
				}
			}
		)*
	}
}

impl_number_deserialise! {
	u8: marker_is_valid_u8
	u16: marker_is_valid_u16
	u32: marker_is_valid_u32
	u64: marker_is_valid_u64
	u128: marker_is_valid_u128

	i8: marker_is_valid_i8
	i16: marker_is_valid_i16
	i32: marker_is_valid_i32
	i64: marker_is_valid_i64
	i128: marker_is_valid_i128
}
