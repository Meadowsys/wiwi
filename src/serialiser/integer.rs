#![allow(clippy::match_overlapping_arm)]

// TODO: figure out usize?

use super::{ *, buffer::*, core::*, error::* };
use ::std::ptr;

// max unsigned

pub const MAX_U8_U16:    u16  = 0xff;
pub const MAX_U16_U32:   u32  = 0xff_ff;
pub const MAX_U24_U32:   u32  = 0xff_ff_ff;
pub const MAX_U32_U64:   u64  = 0xff_ff_ff_ff;
pub const MAX_U40_U64:   u64  = 0xff_ff_ff_ff_ff;
pub const MAX_U48_U64:   u64  = 0xff_ff_ff_ff_ff_ff;
pub const MAX_U56_U64:   u64  = 0xff_ff_ff_ff_ff_ff_ff;
pub const MAX_U64_U128:  u128 = 0xff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_U72_U128:  u128 = 0xff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_U80_U128:  u128 = 0xff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_U88_U128:  u128 = 0xff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_U96_U128:  u128 = 0xff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_U104_U128: u128 = 0xff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_U112_U128: u128 = 0xff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_U120_U128: u128 = 0xff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_U128_U128: u128 = 0xff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;

pub const MAX_U8_U32: u32 = MAX_U8_U16 as _;

pub const MAX_U8_U64: u64 = MAX_U8_U16 as _;
pub const MAX_U16_U64: u64 = MAX_U16_U32 as _;
pub const MAX_U24_U64: u64 = MAX_U24_U32 as _;

pub const MAX_U8_U128: u128 = MAX_U8_U16 as _;
pub const MAX_U16_U128: u128 = MAX_U16_U32 as _;
pub const MAX_U24_U128: u128 = MAX_U24_U32 as _;
pub const MAX_U32_U128: u128 = MAX_U32_U64 as _;
pub const MAX_U40_U128: u128 = MAX_U40_U64 as _;
pub const MAX_U48_U128: u128 = MAX_U48_U64 as _;
pub const MAX_U56_U128: u128 = MAX_U56_U64 as _;

// sanity check
pub const _: () = assert!(MAX_U8_U16 == u8::MAX as u16);
pub const _: () = assert!(MAX_U16_U32 == u16::MAX as u32);
pub const _: () = assert!(MAX_U32_U64 == u32::MAX as u64);
pub const _: () = assert!(MAX_U64_U128 == u64::MAX as u128);
pub const _: () = assert!(MAX_U128_U128 == u128::MAX);

// max signed

pub const MAX_I8_I16:    i16  = 0x7f;
pub const MAX_I16_I32:   i32  = 0x7f_ff;
pub const MAX_I24_I32:   i32  = 0x7f_ff_ff;
pub const MAX_I32_I64:   i64  = 0x7f_ff_ff_ff;
pub const MAX_I40_I64:   i64  = 0x7f_ff_ff_ff_ff;
pub const MAX_I48_I64:   i64  = 0x7f_ff_ff_ff_ff_ff;
pub const MAX_I56_I64:   i64  = 0x7f_ff_ff_ff_ff_ff_ff;
pub const MAX_I64_I128:  i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_I72_I128:  i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_I80_I128:  i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_I88_I128:  i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_I96_I128:  i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_I104_I128: i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_I112_I128: i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_I120_I128: i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;
pub const MAX_I128_I128: i128 = 0x7f_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff_ff;

pub const MAX_I8_I32: i32 = MAX_I8_I16 as _;

pub const MAX_I8_I64: i64 = MAX_I8_I16 as _;
pub const MAX_I16_I64: i64 = MAX_I16_I32 as _;
pub const MAX_I24_I64: i64 = MAX_I24_I32 as _;

pub const MAX_I8_I128: i128 = MAX_I8_I16 as _;
pub const MAX_I16_I128: i128 = MAX_I16_I32 as _;
pub const MAX_I24_I128: i128 = MAX_I24_I32 as _;
pub const MAX_I32_I128: i128 = MAX_I32_I64 as _;
pub const MAX_I40_I128: i128 = MAX_I40_I64 as _;
pub const MAX_I48_I128: i128 = MAX_I48_I64 as _;
pub const MAX_I56_I128: i128 = MAX_I56_I64 as _;

// sanity check
pub const _: () = assert!(MAX_I8_I16 == i8::MAX as i16);
pub const _: () = assert!(MAX_I16_I32 == i16::MAX as i32);
pub const _: () = assert!(MAX_I32_I64 == i32::MAX as i64);
pub const _: () = assert!(MAX_I64_I128 == i64::MAX as i128);
pub const _: () = assert!(MAX_I128_I128 == i128::MAX);

// min signed

pub const MIN_I8_I16: i16  = !MAX_I8_I16;
pub const MIN_I16_I32: i32  = !MAX_I16_I32;
pub const MIN_I24_I32: i32  = !MAX_I24_I32;
pub const MIN_I32_I64: i64  = !MAX_I32_I64;
pub const MIN_I40_I64: i64  = !MAX_I40_I64;
pub const MIN_I48_I64: i64  = !MAX_I48_I64;
pub const MIN_I56_I64: i64  = !MAX_I56_I64;
pub const MIN_I64_I128: i128 = !MAX_I64_I128;
pub const MIN_I72_I128: i128 = !MAX_I72_I128;
pub const MIN_I80_I128: i128 = !MAX_I80_I128;
pub const MIN_I88_I128: i128 = !MAX_I88_I128;
pub const MIN_I96_I128: i128 = !MAX_I96_I128;
pub const MIN_I104_I128: i128 = !MAX_I104_I128;
pub const MIN_I112_I128: i128 = !MAX_I112_I128;
pub const MIN_I120_I128: i128 = !MAX_I120_I128;
pub const MIN_I128_I128: i128 = !MAX_I128_I128;

pub const MIN_I8_I32: i32 = !MAX_I8_I32;

pub const MIN_I8_I64: i64 = !MAX_I8_I64;
pub const MIN_I16_I64: i64 = !MAX_I16_I64;
pub const MIN_I24_I64: i64 = !MAX_I24_I64;

pub const MIN_I8_I128: i128 = !MAX_I8_I128;
pub const MIN_I16_I128: i128 = !MAX_I16_I128;
pub const MIN_I24_I128: i128 = !MAX_I24_I128;
pub const MIN_I32_I128: i128 = !MAX_I32_I128;
pub const MIN_I40_I128: i128 = !MAX_I40_I128;
pub const MIN_I48_I128: i128 = !MAX_I48_I128;
pub const MIN_I56_I128: i128 = !MAX_I56_I128;

// sanity check
pub const _: () = assert!(MIN_I8_I16 == i8::MIN as i16);
pub const _: () = assert!(MIN_I16_I32 == i16::MIN as i32);
pub const _: () = assert!(MIN_I32_I64 == i32::MIN as i64);
pub const _: () = assert!(MIN_I64_I128 == i64::MIN as i128);
pub const _: () = assert!(MIN_I128_I128 == i128::MIN);

exported_match_macro!(fits_in_smallint_positive: 0..=127);
exported_match_macro!(fits_in_smallint_negative: -64..=-1);
exported_match_macro!(fits_in_smallint_unsigned: 0..=127);
exported_match_macro!(fits_in_smallint_signed: -64..=127);
exported_match_macro!(fits_in_smallint_signed_as_unsigned_range: 0..=127 | 192..=255);

// unsigned

exported_match_macro!(fits_in_u8_u16: ..=MAX_U8_U16);
exported_match_macro!(fits_in_u16_u32: ..=MAX_U16_U32);
exported_match_macro!(fits_in_u24_u32: ..=MAX_U24_U32);
exported_match_macro!(fits_in_u32_u64: ..=MAX_U32_U64);
exported_match_macro!(fits_in_u40_u64: ..=MAX_U40_U64);
exported_match_macro!(fits_in_u48_u64: ..=MAX_U48_U64);
exported_match_macro!(fits_in_u56_u64: ..=MAX_U56_U64);
exported_match_macro!(fits_in_u64_u128: ..=MAX_U64_U128);
exported_match_macro!(fits_in_u72_u128: ..=MAX_U72_U128);
exported_match_macro!(fits_in_u80_u128: ..=MAX_U80_U128);
exported_match_macro!(fits_in_u88_u128: ..=MAX_U88_U128);
exported_match_macro!(fits_in_u96_u128: ..=MAX_U96_U128);
exported_match_macro!(fits_in_u104_u128: ..=MAX_U104_U128);
exported_match_macro!(fits_in_u112_u128: ..=MAX_U112_U128);
exported_match_macro!(fits_in_u120_u128: ..=MAX_U120_U128);
exported_match_macro!(fits_in_u128_u128: ..=MAX_U128_U128);

exported_match_macro!(fits_in_u8_u32: ..=MAX_U8_U32);

exported_match_macro!(fits_in_u8_u64: ..=MAX_U8_U64);
exported_match_macro!(fits_in_u16_u64: ..=MAX_U16_U64);
exported_match_macro!(fits_in_u24_u64: ..=MAX_U24_U64);

exported_match_macro!(fits_in_u8_u128: ..=MAX_U8_U128);
exported_match_macro!(fits_in_u16_u128: ..=MAX_U16_U128);
exported_match_macro!(fits_in_u24_u128: ..=MAX_U24_U128);
exported_match_macro!(fits_in_u32_u128: ..=MAX_U32_U128);
exported_match_macro!(fits_in_u40_u128: ..=MAX_U40_U128);
exported_match_macro!(fits_in_u48_u128: ..=MAX_U48_U128);
exported_match_macro!(fits_in_u56_u128: ..=MAX_U56_U128);

// signed

exported_match_macro!(fits_in_i8_i16: MIN_I8_I16..=MAX_I8_I16);
exported_match_macro!(fits_in_i16_i32: MIN_I16_I32..=MAX_I16_I32);
exported_match_macro!(fits_in_i24_i32: MIN_I24_I32..=MAX_I24_I32);
exported_match_macro!(fits_in_i32_i64: MIN_I32_I64..=MAX_I32_I64);
exported_match_macro!(fits_in_i40_i64: MIN_I40_I64..=MAX_I40_I64);
exported_match_macro!(fits_in_i48_i64: MIN_I48_I64..=MAX_I48_I64);
exported_match_macro!(fits_in_i56_i64: MIN_I56_I64..=MAX_I56_I64);
exported_match_macro!(fits_in_i64_i128: MIN_I64_I128..=MAX_I64_I128);
exported_match_macro!(fits_in_i72_i128: MIN_I72_I128..=MAX_I72_I128);
exported_match_macro!(fits_in_i80_i128: MIN_I80_I128..=MAX_I80_I128);
exported_match_macro!(fits_in_i88_i128: MIN_I88_I128..=MAX_I88_I128);
exported_match_macro!(fits_in_i96_i128: MIN_I96_I128..=MAX_I96_I128);
exported_match_macro!(fits_in_i104_i128: MIN_I104_I128..=MAX_I104_I128);
exported_match_macro!(fits_in_i112_i128: MIN_I112_I128..=MAX_I112_I128);
exported_match_macro!(fits_in_i120_i128: MIN_I120_I128..=MAX_I120_I128);
exported_match_macro!(fits_in_i128_i128: MIN_I128_I128..=MAX_I128_I128);

exported_match_macro!(fits_in_i8_i32: MIN_I8_I32..=MAX_I8_I32);

exported_match_macro!(fits_in_i8_i64: MIN_I8_I64..=MAX_I8_I64);
exported_match_macro!(fits_in_i16_i64: MIN_I16_I64..=MAX_I16_I64);
exported_match_macro!(fits_in_i24_i64: MIN_I24_I64..=MAX_I24_I64);

exported_match_macro!(fits_in_i8_i128: MIN_I8_I128..=MAX_I8_I128);
exported_match_macro!(fits_in_i16_i128: MIN_I16_I128..=MAX_I16_I128);
exported_match_macro!(fits_in_i24_i128: MIN_I24_I128..=MAX_I24_I128);
exported_match_macro!(fits_in_i32_i128: MIN_I32_I128..=MAX_I32_I128);
exported_match_macro!(fits_in_i40_i128: MIN_I40_I128..=MAX_I40_I128);
exported_match_macro!(fits_in_i48_i128: MIN_I48_I128..=MAX_I48_I128);
exported_match_macro!(fits_in_i56_i128: MIN_I56_I128..=MAX_I56_I128);

// markers

pub const INT_MARKER_BASE: u8 = 0b100_00000;

pub const MARKER_U8: u8 = INT_MARKER_BASE;
pub const MARKER_U16: u8 = MARKER_U8 + 2;
pub const MARKER_U24: u8 = MARKER_U16 + 2;
pub const MARKER_U32: u8 = MARKER_U24 + 2;
pub const MARKER_U40: u8 = MARKER_U32 + 2;
pub const MARKER_U48: u8 = MARKER_U40 + 2;
pub const MARKER_U56: u8 = MARKER_U48 + 2;
pub const MARKER_U64: u8 = MARKER_U56 + 2;
pub const MARKER_U72: u8 = MARKER_U64 + 2;
pub const MARKER_U80: u8 = MARKER_U72 + 2;
pub const MARKER_U88: u8 = MARKER_U80 + 2;
pub const MARKER_U96: u8 = MARKER_U88 + 2;
pub const MARKER_U104: u8 = MARKER_U96 + 2;
pub const MARKER_U112: u8 = MARKER_U104 + 2;
pub const MARKER_U120: u8 = MARKER_U112 + 2;
pub const MARKER_U128: u8 = MARKER_U120 + 2;

pub const MARKER_I8: u8 = MARKER_U8 + 1;
pub const MARKER_I16: u8 = MARKER_U16 + 1;
pub const MARKER_I24: u8 = MARKER_U24 + 1;
pub const MARKER_I32: u8 = MARKER_U32 + 1;
pub const MARKER_I40: u8 = MARKER_U40 + 1;
pub const MARKER_I48: u8 = MARKER_U48 + 1;
pub const MARKER_I56: u8 = MARKER_U56 + 1;
pub const MARKER_I64: u8 = MARKER_U64 + 1;
pub const MARKER_I72: u8 = MARKER_U72 + 1;
pub const MARKER_I80: u8 = MARKER_U80 + 1;
pub const MARKER_I88: u8 = MARKER_U88 + 1;
pub const MARKER_I96: u8 = MARKER_U96 + 1;
pub const MARKER_I104: u8 = MARKER_U104 + 1;
pub const MARKER_I112: u8 = MARKER_U112 + 1;
pub const MARKER_I120: u8 = MARKER_U120 + 1;
pub const MARKER_I128: u8 = MARKER_U128 + 1;

fn min_marker_for_u8(num: u8) -> u8 {
	match num {
		num @ fits_in_smallint_unsigned!() => { num }
		_ => { MARKER_U8 }
	}
}

fn min_marker_for_u16(num: u16) -> u8 {
	match num {
		num @ fits_in_smallint_unsigned!() => { num as _ }
		fits_in_u8_u16!() => { MARKER_U8 }
		_ => { MARKER_U16 }
	}
}

fn min_marker_for_u32(num: u32) -> u8 {
	match num {
		num @ fits_in_smallint_unsigned!() => { num as _ }
		fits_in_u8_u32!() => { MARKER_U8 }
		fits_in_u16_u32!() => { MARKER_U16 }
		fits_in_u24_u32!() => { MARKER_U24 }
		_ => { MARKER_U32 }
	}
}

fn min_marker_for_u64(num: u64) -> u8 {
	match num {
		num @ fits_in_smallint_unsigned!() => { num as _ }
		fits_in_u8_u64!() => { MARKER_U8 }
		fits_in_u16_u64!() => { MARKER_U16 }
		fits_in_u24_u64!() => { MARKER_U24 }
		fits_in_u32_u64!() => { MARKER_U32 }
		fits_in_u40_u64!() => { MARKER_U40 }
		fits_in_u48_u64!() => { MARKER_U48 }
		fits_in_u56_u64!() => { MARKER_U56 }
		_ => { MARKER_U64 }
	}
}

fn min_marker_for_u128(num: u128) -> u8 {
	match num {
		num @ fits_in_smallint_unsigned!() => { num as _ }
		fits_in_u8_u128!() => { MARKER_U8 }
		fits_in_u16_u128!() => { MARKER_U16 }
		fits_in_u24_u128!() => { MARKER_U24 }
		fits_in_u32_u128!() => { MARKER_U32 }
		fits_in_u40_u128!() => { MARKER_U40 }
		fits_in_u48_u128!() => { MARKER_U48 }
		fits_in_u56_u128!() => { MARKER_U56 }
		fits_in_u64_u128!() => { MARKER_U64 }
		fits_in_u72_u128!() => { MARKER_U72 }
		fits_in_u80_u128!() => { MARKER_U80 }
		fits_in_u88_u128!() => { MARKER_U88 }
		fits_in_u96_u128!() => { MARKER_U96 }
		fits_in_u104_u128!() => { MARKER_U104 }
		fits_in_u112_u128!() => { MARKER_U112 }
		fits_in_u120_u128!() => { MARKER_U120 }
		_ => { MARKER_U128 }
	}
}

fn min_marker_for_i8(num: i8) -> u8 {
	match num {
		num @ fits_in_smallint_signed!() => { num as _ }
		_ => { MARKER_I8 }
	}
}

fn min_marker_for_i16(num: i16) -> u8 {
	match num {
		num @ fits_in_smallint_signed!() => { num as _ }
		fits_in_i8_i16!() => { MARKER_I8 }
		_ => { MARKER_I16 }
	}
}

fn min_marker_for_i32(num: i32) -> u8 {
	match num {
		num @ fits_in_smallint_signed!() => { num as _ }
		fits_in_i8_i32!() => { MARKER_I8 }
		fits_in_i16_i32!() => { MARKER_I16 }
		fits_in_i24_i32!() => { MARKER_I24 }
		_ => { MARKER_I32 }
	}
}

fn min_marker_for_i64(num: i64) -> u8 {
	match num {
		num @ fits_in_smallint_signed!() => { num as _ }
		fits_in_i8_i64!() => { MARKER_I8 }
		fits_in_i16_i64!() => { MARKER_I16 }
		fits_in_i24_i64!() => { MARKER_I24 }
		fits_in_i32_i64!() => { MARKER_I32 }
		fits_in_i40_i64!() => { MARKER_I40 }
		fits_in_i48_i64!() => { MARKER_I48 }
		fits_in_i56_i64!() => { MARKER_I56 }
		_ => { MARKER_I64 }
	}
}

fn min_marker_for_i128(num: i128) -> u8 {
	match num {
		num @ fits_in_smallint_signed!() => { num as _ }
		fits_in_i8_i128!() => { MARKER_I8 }
		fits_in_i16_i128!() => { MARKER_I16 }
		fits_in_i24_i128!() => { MARKER_I24 }
		fits_in_i32_i128!() => { MARKER_I32 }
		fits_in_i40_i128!() => { MARKER_I40 }
		fits_in_i48_i128!() => { MARKER_I48 }
		fits_in_i56_i128!() => { MARKER_I56 }
		fits_in_i64_i128!() => { MARKER_I64 }
		fits_in_i72_i128!() => { MARKER_I72 }
		fits_in_i80_i128!() => { MARKER_I80 }
		fits_in_i88_i128!() => { MARKER_I88 }
		fits_in_i96_i128!() => { MARKER_I96 }
		fits_in_i104_i128!() => { MARKER_I104 }
		fits_in_i112_i128!() => { MARKER_I112 }
		fits_in_i120_i128!() => { MARKER_I120 }
		_ => { MARKER_I128 }
	}
}

macro_rules! impl_number_serialise {
	($($num:ty: $min_marker:ident)*) => {
		$(
			impl Serialise for $num {
				fn serialise<B: BufferWrite>(&self, output: &mut B, options: &Options) {
					const BYTES: usize = <$num>::BITS as usize / 8;
					let bytes = self.to_le_bytes();
					let marker = $min_marker(*self);

					unsafe {
						if marker >> 5 == 0b100 {
							let count = (((marker & 0b11111) >> 1) + 1) as usize;
							let total = count + 1;

							output.reserve(total);
							output.with_ptr(|ptr| {
								ptr::write(ptr, marker);
								ptr::copy_nonoverlapping(&bytes as *const u8, ptr.add(1), count);
								total
							});
						} else {
							output.reserve(1);
							output.with_ptr(|ptr| {
								ptr::write(ptr, marker);
								1
							});
						}
					}
				}
			}
		)*
	}
}

impl_number_serialise! {
	u8: min_marker_for_u8
	u16: min_marker_for_u16
	u32: min_marker_for_u32
	u64: min_marker_for_u64
	u128: min_marker_for_u128


	i8: min_marker_for_i8
	i16: min_marker_for_i16
	i32: min_marker_for_i32
	i64: min_marker_for_i64
	i128: min_marker_for_i128
}

pub enum CheckedIntMarker {
	SmallInt(u8),
	Read(usize)
}

macro_rules! marker_valid_for_inner {
	($num:ty {
		// name of output fn
		$fn_name:ident
		// macro to use to match smallints
		$smallint_macro:ident
		// target size (ie. 2 if deserialising into a u16)
		$dest_size:literal

		// reason why we're giving _all_ markers:
		// hopefully the optimiser can figure out "5 <= 4" will always be false
		// and get rid of that match arm

		// all marker lists
		{
			// marker: self explanatory
			// marker_type: type the marker encodes as str literal (for err message generation, ie. "u48")
			// marker_size: size of the encoded type as int literal
			$(($marker:ident, $marker_type:literal, $marker_size:literal))*
		}

		// present this block if the target should be unsigned
		// inside should go all signed markers, for err messages
		$(should be unsigned {
			// marker_signed: the signed marker
			// target_signed: type the marker represents, as str literal
			$(($marker_signed:ident, $target_signed:literal))*
		})?

		// if the target should be signed
		// isnide should go all unsigned markers for err messages
		$(should be signed {
			// marker_unsigned: unsigned marker
			// target_unsigned: type the marker represents, as str literal
			$(($marker_unsigned:ident, $target_unsigned:literal))*
		})?
	}) => {
		pub fn $fn_name(marker: u8) -> Result<CheckedIntMarker> {
			#[deny(unreachable_patterns)]
			Ok(match marker {
				int @ $smallint_macro!() => { CheckedIntMarker::SmallInt(int) }

				$(
					$marker if $marker_size <= $dest_size => { CheckedIntMarker::Read($marker_size) }
					$marker => {
						return err(concat!($marker_type, " will overflow ", stringify!($num)))
					}
				)*

				$(
					$(
						$marker_signed => {
							return err(concat!("expected unsigned number, found ", $target_signed))
						}
					)*
					_ => { return err(concat!("expected ", stringify!($num), "-compatible unsigned number")) }
				)?

				$(
					$(
						$marker_unsigned => {
							return err(concat!("expected signed number, found ", $target_unsigned))
						}
					)*
					_ => { return err(concat!("expected ", stringify!($num), "-compatible signed number")) }
				)?
			})
		}
	}
}

macro_rules! marker_valid_for_unsigned {
	($($num:ty {
		$fn_name:ident
		$smallint_macro:ident
		$dest_size:literal
	})*) => {
		$(
			marker_valid_for_inner! {
				$num {
					$fn_name
					$smallint_macro
					$dest_size

					{
						(MARKER_U8, "u8", 1)
						(MARKER_U16, "u16", 2)
						(MARKER_U24, "u24", 3)
						(MARKER_U32, "u32", 4)
						(MARKER_U40, "u40", 5)
						(MARKER_U48, "u48", 6)
						(MARKER_U56, "u56", 7)
						(MARKER_U64, "u64", 8)
						(MARKER_U72, "u72", 9)
						(MARKER_U80, "u80", 10)
						(MARKER_U88, "u88", 11)
						(MARKER_U96, "u96", 12)
						(MARKER_U104, "u104", 13)
						(MARKER_U112, "u112", 14)
						(MARKER_U120, "u120", 15)
						(MARKER_U128, "u128", 16)
					}

					should be unsigned {
						(MARKER_I8, "i8")
						(MARKER_I16, "i16")
						(MARKER_I24, "i24")
						(MARKER_I32, "i32")
						(MARKER_I40, "i40")
						(MARKER_I48, "i48")
						(MARKER_I56, "i56")
						(MARKER_I64, "i64")
						(MARKER_I72, "i72")
						(MARKER_I80, "i80")
						(MARKER_I88, "i88")
						(MARKER_I96, "i96")
						(MARKER_I104, "i104")
						(MARKER_I112, "i112")
						(MARKER_I120, "i120")
						(MARKER_I128, "i128")
					}
				}
			}
		)*
	}
}

marker_valid_for_unsigned! {
	u8 {
		marker_valid_for_u8
		fits_in_smallint_unsigned
		1
	}
	u16 {
		marker_valid_for_u16
		fits_in_smallint_unsigned
		2
	}
	u32 {
		marker_valid_for_u32
		fits_in_smallint_unsigned
		4
	}
	u64 {
		marker_valid_for_u64
		fits_in_smallint_unsigned
		8
	}
	u128 {
		marker_valid_for_u128
		fits_in_smallint_unsigned
		16
	}
}

macro_rules! marker_valid_for_signed {
	($($num:ty {
		$fn_name:ident
		$smallint_macro:ident
		$dest_size:literal
	})*) => {
		$(
			marker_valid_for_inner! {
				$num {
					$fn_name
					$smallint_macro
					$dest_size

					{
						(MARKER_I8, "i8", 1)
						(MARKER_I16, "i16", 2)
						(MARKER_I24, "i24", 3)
						(MARKER_I32, "i32", 4)
						(MARKER_I40, "i40", 5)
						(MARKER_I48, "i48", 6)
						(MARKER_I56, "i56", 7)
						(MARKER_I64, "i64", 8)
						(MARKER_I72, "i72", 9)
						(MARKER_I80, "i80", 10)
						(MARKER_I88, "i88", 11)
						(MARKER_I96, "i96", 12)
						(MARKER_I104, "i104", 13)
						(MARKER_I112, "i112", 14)
						(MARKER_I120, "i120", 15)
						(MARKER_I128, "i128", 16)
					}

					should be signed {
						(MARKER_U8, "u8")
						(MARKER_U16, "u16")
						(MARKER_U24, "u24")
						(MARKER_U32, "u32")
						(MARKER_U40, "u40")
						(MARKER_U48, "u48")
						(MARKER_U56, "u56")
						(MARKER_U64, "u64")
						(MARKER_U72, "u72")
						(MARKER_U80, "u80")
						(MARKER_U88, "u88")
						(MARKER_U96, "u96")
						(MARKER_U104, "u104")
						(MARKER_U112, "u112")
						(MARKER_U120, "u120")
						(MARKER_U128, "u128")
					}
				}
			}
		)*
	}
}

marker_valid_for_signed! {
	i8 {
		marker_valid_for_i8
		fits_in_smallint_signed_as_unsigned_range
		1
	}
	i16 {
		marker_valid_for_i16
		fits_in_smallint_signed_as_unsigned_range
		2
	}
	i32 {
		marker_valid_for_i32
		fits_in_smallint_signed_as_unsigned_range
		4
	}
	i64 {
		marker_valid_for_i64
		fits_in_smallint_signed_as_unsigned_range
		8
	}
	i128 {
		marker_valid_for_i128
		fits_in_smallint_signed_as_unsigned_range
		16
	}
}

fn deserialise_rest_of_inner_to_le<'h, const BYTES: usize, const SIGN_EXTEND: bool, B: BufferRead<'h>>(
	result: CheckedIntMarker,
	input: &mut B
) -> Result<[u8; BYTES]> {
	use CheckedIntMarker::*;

	let mut array = [0u8; BYTES];
	let ptr = &mut array as *mut u8;

	match result {
		SmallInt(byte) => unsafe {
			ptr::write(ptr, byte);

			if SIGN_EXTEND {
				let sign = if byte >> 7 == 0 { 0 } else { u8::MAX };
				ptr::write_bytes(ptr.add(1), sign, BYTES - 1);
			}
		}

		Read(count) => unsafe {
			// we don't have markers for zero sized ints lol
			// the closest to "zero sized int" we have is smallint, which is handled
			// in the previous match arm
			debug_assert!(count > 0);

			// otherwise we copy too much (also it should have caught this as overflow)
			// and returned `Invalid`, which will be handled by the next match arm
			debug_assert!(count <= BYTES);

			let input_ptr = input.read_bytes_ptr(count)?;
			ptr::copy_nonoverlapping(input_ptr, ptr, count);

			if SIGN_EXTEND {
				// count myst be gte 1 so this will always be valid
				let last_byte = *input_ptr.add(count).sub(1);
				let sign = if last_byte >> 7 == 0 { 0 } else { u8::MAX };
				ptr::write_bytes(ptr.add(count), sign, BYTES - count);
			}
		}
	}

	Ok(array)
}

macro_rules! deserialise_rest_of {
	($($num:ty: $fn_name:ident, $num_size:literal, $signed:literal
	)*) => {
		$(
			pub fn $fn_name<'h, B: BufferRead<'h>>(checked_marker: CheckedIntMarker, input: &mut B) -> Result<$num> {
				let rest = deserialise_rest_of_inner_to_le::<$num_size, $signed, _>(checked_marker, input)?;
				Ok(<$num>::from_le_bytes(rest))
			}
		)*
	}
}

deserialise_rest_of! {
	u8: deserialise_rest_of_u8, 1, false
	u16: deserialise_rest_of_u16, 2, false
	u32: deserialise_rest_of_u32, 4, false
	u64: deserialise_rest_of_u64, 8, false
	u128: deserialise_rest_of_u128, 16, false

	i8: deserialise_rest_of_i8, 1, true
	i16: deserialise_rest_of_i16, 2, true
	i32: deserialise_rest_of_i32, 4, true
	i64: deserialise_rest_of_i64, 8, true
	i128: deserialise_rest_of_i128, 16, true
}

macro_rules! impl_number_deserialise {
	($($num:ty: $marker_valid:ident, $deserialise_rest:ident)*) => {
		$(
			impl<'h> Deserialise<'h> for $num {
				fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
					let marker = input.read_byte()?;
					let checked_marker = $marker_valid(marker)?;
					$deserialise_rest(checked_marker, input)
				}
			}
		)*
	}
}

impl_number_deserialise! {
	u8: marker_valid_for_u8, deserialise_rest_of_u8
	u16: marker_valid_for_u16, deserialise_rest_of_u16
	u32: marker_valid_for_u32, deserialise_rest_of_u32
	u64: marker_valid_for_u64, deserialise_rest_of_u64
	u128: marker_valid_for_u128, deserialise_rest_of_u128

	i8: marker_valid_for_i8, deserialise_rest_of_i8
	i16: marker_valid_for_i16, deserialise_rest_of_i16
	i32: marker_valid_for_i32, deserialise_rest_of_i32
	i64: marker_valid_for_i64, deserialise_rest_of_i64
	i128: marker_valid_for_i128, deserialise_rest_of_i128
}
