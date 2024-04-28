#![allow(clippy::match_overlapping_arm)]

use super::{ *, buffer::*, error::* };
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
pub const _: () = assert!(MAX_U8_U16 == u8::MAX as _);
pub const _: () = assert!(MAX_U16_U32 == u16::MAX as _);
pub const _: () = assert!(MAX_U32_U64 == u32::MAX as _);
pub const _: () = assert!(MAX_U64_U128 == u64::MAX as _);
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
pub const _: () = assert!(MAX_I8_I16 == i8::MAX as _);
pub const _: () = assert!(MAX_I16_I32 == i16::MAX as _);
pub const _: () = assert!(MAX_I32_I64 == i32::MAX as _);
pub const _: () = assert!(MAX_I64_I128 == i64::MAX as _);
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
pub const _: () = assert!(MIN_I8_I16 == i8::MIN as _);
pub const _: () = assert!(MIN_I16_I32 == i16::MIN as _);
pub const _: () = assert!(MIN_I32_I64 == i32::MIN as _);
pub const _: () = assert!(MIN_I64_I128 == i64::MIN as _);
pub const _: () = assert!(MIN_I128_I128 == i128::MIN);

exported_match_macro!(match_smallint_positive: 0..=127);
exported_match_macro!(match_smallint_negative: -64..=-1);
exported_match_macro!(match_smallint_unsigned: 0..=127);
exported_match_macro!(match_smallint_signed: -64..=127);

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
		num @ match_smallint_unsigned!() => { num }
		_ => { MARKER_U8 }
	}
}

fn min_marker_for_u16(num: u16) -> u8 {
	match num {
		num @ match_smallint_unsigned!() => { num as _ }
		fits_in_u8_u16!() => { MARKER_U8 }
		_ => { MARKER_U16 }
	}
}

fn min_marker_for_u32(num: u32) -> u8 {
	match num {
		num @ match_smallint_unsigned!() => { num as _ }
		fits_in_u8_u32!() => { MARKER_U8 }
		fits_in_u16_u32!() => { MARKER_U16 }
		fits_in_u24_u32!() => { MARKER_U24 }
		_ => { MARKER_U32 }
	}
}

fn min_marker_for_u64(num: u64) -> u8 {
	match num {
		num @ match_smallint_unsigned!() => { num as _ }
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
		num @ match_smallint_unsigned!() => { num as _ }
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
		num @ match_smallint_signed!() => { num as _ }
		_ => { MARKER_I8 }
	}
}

fn min_marker_for_i16(num: i16) -> u8 {
	match num {
		num @ match_smallint_signed!() => { num as _ }
		fits_in_i8_i16!() => { MARKER_I8 }
		_ => { MARKER_I16 }
	}
}

fn min_marker_for_i32(num: i32) -> u8 {
	match num {
		num @ match_smallint_signed!() => { num as _ }
		fits_in_i8_i32!() => { MARKER_I8 }
		fits_in_i16_i32!() => { MARKER_I16 }
		fits_in_i24_i32!() => { MARKER_I24 }
		_ => { MARKER_I32 }
	}
}

fn min_marker_for_i64(num: i64) -> u8 {
	match num {
		num @ match_smallint_signed!() => { num as _ }
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
		num @ match_smallint_signed!() => { num as _ }
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

					output.reserve(BYTES + 1);
					unsafe { output.with_ptr(|ptr| {
						ptr::write(ptr, marker);

						if marker >> 5 == 0b100 {
							let count = (((marker & 0b11111) >> 1) + 1) as usize;
							ptr::copy_nonoverlapping(&bytes as *const u8, ptr.add(1), count);
							return count + 1
						}

						1
					}) }
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

// pub fn serialise_int<N, B>(num: N, output: &mut B)
// where
// 	N: Num,
// 	B: BufferWrite
// {
// 	let bytes = num.to_le_bytes();
// 	let marker = num.min_marker_for();

// 	output.reserve(N::BYTES + 1);
// 	unsafe { output.with_ptr(|ptr| {
// 		ptr::write(ptr, marker);

// 		if marker >> 5 == 0b100 {
// 			let count = (((marker & 0b11111) >> 1) + 1) as usize;
// 			ptr::copy_nonoverlapping(&bytes as *const u8, ptr.add(1), count);
// 			return count + 1
// 		}

// 		1
// 	}) }
// }





















// fn deserialise_rest_of_int_no_smallint<const BYTES: usize, const SIGNED: bool, N, B>(marker: u8, input: &mut B)
// where
// 	N: Num<BYTES, SIGNED>,
// 	B: BufferRead
// {
// 	// let bytes = [0u8; ]
// 	// if marker >>
// }














// fn deserialise_rest_of_number<const BYTES: usize, const SIGNED: bool, N, B>(marker: u8, input: &mut B)
// where
// 	N: Num<BYTES, SIGNED>,
// 	B: BufferRead
// {
// 	// let bytes = [0u8; ]
// 	if marker >>
// }

// pub struct NumSerialiser<const BYTES: usize, const SIGNED: bool> {
// 	bytes: [u8; BYTES],
// 	marker: u8,
// 	amount_to_write: usize
// }

// impl<const BYTES: usize, const SIGNED: bool> NumSerialiser<BYTES, SIGNED> {
// 	fn new<N>(num: N) -> Self
// 	where
// 		N: Num<BYTES, SIGNED>
// 	{
// 		let mut bytes = num.to_le_bytes();
// 		let marker = num.get_marker_for();
// 		let amount_to_write = if marker >> 5 == 0b100 {
// 			(((marker & 0b11111) >> 1) + 1) as usize
// 		} else {
// 			0
// 		};

// 		Self { bytes, marker, amount_to_write }
// 	}
// }

// impl<'h, const BYTES: usize> SerialiseIntermediate<'h> for NumSerialiser<BYTES> {
// 	fn serialise<B: BufferWrite>(&mut self, output: &mut B) {
// 		// output.write_byte(self.marker);
// 	}
// }

// // TODO: these is_valid functions might be able to be optimised
// // I think something can be done with that special property of markers that
// // we rely on elsewhere (the `marker >> 1 == amount` of bytes after thing)
//
// pub fn marker_is_valid_u8(marker: u8) -> bool {
// 	marker == MARKER_U8
// }
//
// pub fn marker_is_valid_u16(marker: u8) -> bool {
// 	match marker {
// 		MARKER_U16 => { true }
// 		_ => { marker_is_valid_u8(marker) }
// 	}
// }
//
// pub fn marker_is_valid_u32(marker: u8) -> bool {
// 	match marker {
// 		MARKER_U24 => { true }
// 		MARKER_U32 => { true }
// 		_ => { marker_is_valid_u16(marker) }
// 	}
// }
//
// pub fn marker_is_valid_u64(marker: u8) -> bool {
// 	match marker {
// 		MARKER_U40 => { true }
// 		MARKER_U48 => { true }
// 		MARKER_U56 => { true }
// 		MARKER_U64 => { true }
// 		_ => { marker_is_valid_u32(marker) }
// 	}
// }
//
// pub fn marker_is_valid_u128(marker: u8) -> bool {
// 	match marker {
// 		MARKER_U72 => { true }
// 		MARKER_U80 => { true }
// 		MARKER_U88 => { true }
// 		MARKER_U96 => { true }
// 		MARKER_U104 => { true }
// 		MARKER_U112 => { true }
// 		MARKER_U120 => { true }
// 		MARKER_U128 => { true }
// 		_ => { marker_is_valid_u64(marker) }
// 	}
// }
//
// pub fn marker_is_valid_i8(marker: u8) -> bool {
// 	marker == MARKER_I8
// }
//
// pub fn marker_is_valid_i16(marker: u8) -> bool {
// 	match marker {
// 		MARKER_I16 => { true }
// 		_ => { marker_is_valid_i8(marker) }
// 	}
// }
//
// pub fn marker_is_valid_i32(marker: u8) -> bool {
// 	match marker {
// 		MARKER_I24 => { true }
// 		MARKER_I32 => { true }
// 		_ => { marker_is_valid_i16(marker) }
// 	}
// }
//
// pub fn marker_is_valid_i64(marker: u8) -> bool {
// 	match marker {
// 		MARKER_I40 => { true }
// 		MARKER_I48 => { true }
// 		MARKER_I56 => { true }
// 		MARKER_I64 => { true }
// 		_ => { marker_is_valid_i32(marker) }
// 	}
// }
//
// pub fn marker_is_valid_i128(marker: u8) -> bool {
// 	match marker {
// 		MARKER_I72 => { true }
// 		MARKER_I80 => { true }
// 		MARKER_I88 => { true }
// 		MARKER_I96 => { true }
// 		MARKER_I104 => { true }
// 		MARKER_I112 => { true }
// 		MARKER_I120 => { true }
// 		MARKER_I128 => { true }
// 		_ => { marker_is_valid_i64(marker) }
// 	}
// }
//
// macro_rules! num_serialise_rest_fns {
// 	($($num:ty: $fn_name:ident)*) => {
// 		$(
// 			/// # Safety
// 			///
// 			/// The passed in marker must be valid for the int type; otherwise,
// 			/// invalid memory can be read from, breaking memory safety
// 			pub unsafe fn $fn_name<B: BufferImplWrite>(num: $num, marker: u8, output: &mut B) {
// 				let bytes = <$num>::to_le_bytes(num);
// 				output.write_slice(slice::from_raw_parts(
// 					&bytes as *const u8,
// 					(marker >> 1) as usize
// 				));
// 			}
// 		)*
// 	}
// }
//
// num_serialise_rest_fns! {
// 	u8: serialise_rest_of_u8
// 	u16: serialise_rest_of_u16
// 	u32: serialise_rest_of_u32
// 	u64: serialise_rest_of_u64
// 	u128: serialise_rest_of_u128
//
// 	i8: serialise_rest_of_i8
// 	i16: serialise_rest_of_i16
// 	i32: serialise_rest_of_i32
// 	i64: serialise_rest_of_i64
// 	i128: serialise_rest_of_i128
// }
//
// macro_rules! impl_number_serialise {
// 	($($num:ty: $rest_fn:ident, $min_marker:ident)*) => {
// 		$(
// 			impl Serialise for $num {
// 				fn serialise<B: BufferImplWrite>(&self, output: &mut B, options: &Options) {
// 					let num = *self;
// 					let marker = $min_marker(num);
//
// 					output.write_byte(marker);
// 					unsafe { $rest_fn(num, marker, output) }
// 				}
// 			}
// 		)*
// 	}
// }
//
// impl_number_serialise! {
// 	u8: serialise_rest_of_u8, min_marker_u8
// 	u16: serialise_rest_of_u16, min_marker_u16
// 	u32: serialise_rest_of_u32, min_marker_u32
// 	u64: serialise_rest_of_u64, min_marker_u64
// 	u128: serialise_rest_of_u128, min_marker_u128
//
// 	i8: serialise_rest_of_i8, min_marker_i8
// 	i16: serialise_rest_of_i16, min_marker_i16
// 	i32: serialise_rest_of_i32, min_marker_i32
// 	i64: serialise_rest_of_i64, min_marker_i64
// 	i128: serialise_rest_of_i128, min_marker_i128
// }
//
// macro_rules! num_deserialise_rest_fns {
// 	($($num:ty: $fn_name:ident, $sign_extend:expr)*) => {
// 		$(
// 			/// # Safety
// 			///
// 			/// The passed in marker must be valid for the int type; otherwise,
// 			/// this function can write to invalid memory.
// 			pub unsafe fn $fn_name<'h, B: BufferImplRead<'h>>(marker: u8, input: &mut B) -> Result<$num> {
// 				const NUM_BYTES: usize = <$num>::BITS as usize / 8;
//
// 				let mut bytes = [0u8; NUM_BYTES];
// 				let bytes_ptr = &mut bytes as *mut u8;
// 				let count = (marker >> 1) as usize;
//
// 				let ptr = input.read_bytes_ptr(count)?;
// 				ptr::copy_nonoverlapping(
// 					ptr,
// 					bytes_ptr,
// 					count
// 				);
//
// 				if $sign_extend {
// 					if count < NUM_BYTES {
// 						let sign_bit = *bytes_ptr.add(count - 1) >> 7;
// 						if sign_bit != 0 {
// 							ptr::write_bytes(
// 								bytes_ptr.add(count),
// 								u8::MAX,
// 								NUM_BYTES - count
// 							);
// 						}
// 					}
// 				}
//
// 				Ok(<$num>::from_le_bytes(bytes))
// 			}
// 		)*
// 	}
// }
//
// num_deserialise_rest_fns! {
// 	u8: deserialise_rest_of_u8, false
// 	u16: deserialise_rest_of_u16, false
// 	u32: deserialise_rest_of_u32, false
// 	u64: deserialise_rest_of_u64, false
// 	u128: deserialise_rest_of_u128, false
//
// 	// i8 will always be one byte only, nothing to extend
// 	i8: deserialise_rest_of_i8, false
// 	i16: deserialise_rest_of_i16, true
// 	i32: deserialise_rest_of_i32, true
// 	i64: deserialise_rest_of_i64, true
// 	i128: deserialise_rest_of_i128, true
// }
//
// macro_rules! impl_number_deserialise {
// 	($($num:ty: $rest_fn:ident, $check_marker_fn:ident)*) => {
// 		$(
// 			impl<'h> Deserialise<'h> for $num {
// 				fn deserialise<B: BufferImplRead<'h>>(input: &mut B) -> Result<Self> {
// 					let marker = input.read_byte()?;
//
// 					if $check_marker_fn(marker) {
// 						unsafe { $rest_fn(marker, input) }
// 					} else {
// 						err(concat!("expected ", stringify!($num), "-compatible integer"))
// 					}
// 				}
// 			}
// 		)*
// 	}
// }
//
// impl_number_deserialise! {
// 	u8: deserialise_rest_of_u8, marker_is_valid_u8
// 	u16: deserialise_rest_of_u16, marker_is_valid_u16
// 	u32: deserialise_rest_of_u32, marker_is_valid_u32
// 	u64: deserialise_rest_of_u64, marker_is_valid_u64
// 	u128: deserialise_rest_of_u128, marker_is_valid_u128
//
// 	i8: deserialise_rest_of_i8, marker_is_valid_i8
// 	i16: deserialise_rest_of_i16, marker_is_valid_i16
// 	i32: deserialise_rest_of_i32, marker_is_valid_i32
// 	i64: deserialise_rest_of_i64, marker_is_valid_i64
// 	i128: deserialise_rest_of_i128, marker_is_valid_i128
// }
