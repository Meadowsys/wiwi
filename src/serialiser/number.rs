use super::{ BufferImpl, marker::* };
use ::std::mem::transmute;
use ::std::slice;

#[inline]
unsafe fn _serialise_num<const MARKER: u8, const N: usize, B: BufferImpl>(
	buf: &mut B,
	ptr: *const u8
) {
	buf.write_byte(MARKER);
	buf.write_slice(slice::from_raw_parts(ptr, N));
}

// TODO: what's better, this? or normal rust formatting? lol
#[inline] unsafe fn _serialise_u8  <B: BufferImpl>(buf: &mut B, num: u8)   { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U8,   1,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u16 <B: BufferImpl>(buf: &mut B, num: u16)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U16,  2,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u24 <B: BufferImpl>(buf: &mut B, num: u32)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U24,  3,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u32 <B: BufferImpl>(buf: &mut B, num: u32)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U32,  4,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u40 <B: BufferImpl>(buf: &mut B, num: u64)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U40,  5,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u48 <B: BufferImpl>(buf: &mut B, num: u64)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U48,  6,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u56 <B: BufferImpl>(buf: &mut B, num: u64)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U56,  7,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u64 <B: BufferImpl>(buf: &mut B, num: u64)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U64,  8,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u72 <B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U72,  9,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u80 <B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U80,  10, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u88 <B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U88,  11, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u96 <B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U96,  12, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u104<B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U104, 13, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u112<B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U112, 14, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u120<B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U120, 15, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_u128<B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_U128, 16, _>(buf, &bytes as *const u8); }

#[inline] unsafe fn _serialise_i8  <B: BufferImpl>(buf: &mut B, num: u8)   { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I8,   1,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i16 <B: BufferImpl>(buf: &mut B, num: u16)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I16,  2,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i24 <B: BufferImpl>(buf: &mut B, num: u32)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I24,  3,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i32 <B: BufferImpl>(buf: &mut B, num: u32)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I32,  4,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i40 <B: BufferImpl>(buf: &mut B, num: u64)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I40,  5,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i48 <B: BufferImpl>(buf: &mut B, num: u64)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I48,  6,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i56 <B: BufferImpl>(buf: &mut B, num: u64)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I56,  7,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i64 <B: BufferImpl>(buf: &mut B, num: u64)  { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I64,  8,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i72 <B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I72,  9,  _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i80 <B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I80,  10, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i88 <B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I88,  11, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i96 <B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I96,  12, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i104<B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I104, 13, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i112<B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I112, 14, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i120<B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I120, 15, _>(buf, &bytes as *const u8); }
#[inline] unsafe fn _serialise_i128<B: BufferImpl>(buf: &mut B, num: u128) { let bytes = num.to_le_bytes(); _serialise_num::<MARKER_I128, 16, _>(buf, &bytes as *const u8); }

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
pub fn serialise_u8<B: BufferImpl>(buf: &mut B, num: u8) {
	unsafe { _serialise_u8(buf, num) }
}

pub fn serialise_u16<B: BufferImpl>(buf: &mut B, num: u16) {
	match num {
		num @ ..=LARGEST_8 => { serialise_u8(buf, num as u8) }
		num => unsafe { _serialise_u16(buf, num) }
	}
}

pub fn serialise_u32<B: BufferImpl>(buf: &mut B, num: u32) {
	match num {
		num @ ..=LARGEST_16 => { serialise_u16(buf, num as u16) }
		num @ ..=LARGEST_24 => unsafe { _serialise_u24(buf, num) }
		num => unsafe { _serialise_u32(buf, num) }
	}
}

pub fn serialise_u64<B: BufferImpl>(buf: &mut B, num: u64) {
	match num {
		num @ ..=LARGEST_32 => { serialise_u32(buf, num as u32) }
		num @ ..=LARGEST_40 => unsafe { _serialise_u40(buf, num) }
		num @ ..=LARGEST_48 => unsafe { _serialise_u48(buf, num) }
		num @ ..=LARGEST_56 => unsafe { _serialise_u56(buf, num) }
		num => unsafe { _serialise_u64(buf, num) }
	}
}

pub fn serialise_u128<B: BufferImpl>(buf: &mut B, num: u128) {
	match num {
		num @ ..=LARGEST_64 => { serialise_u64(buf, num as u64) }
		num @ ..=LARGEST_72 => unsafe { _serialise_u72(buf, num) }
		num @ ..=LARGEST_80 => unsafe { _serialise_u80(buf, num) }
		num @ ..=LARGEST_88 => unsafe { _serialise_u88(buf, num) }
		num @ ..=LARGEST_96 => unsafe { _serialise_u96(buf, num) }
		num @ ..=LARGEST_104 => unsafe { _serialise_u104(buf, num) }
		num @ ..=LARGEST_112 => unsafe { _serialise_u112(buf, num) }
		num @ ..=LARGEST_120 => unsafe { _serialise_u120(buf, num) }
		num => unsafe { _serialise_u128(buf, num) }
	}
}

#[inline]
pub fn serialise_i8<B: BufferImpl>(buf: &mut B, num: i8) {
	unsafe { _serialise_i8(buf, transmute(num)) }
}

pub fn serialise_i16<B: BufferImpl>(buf: &mut B, num: i16) {
	match unsafe { transmute(num) } {
		num @ ..=LARGEST_8 => unsafe { serialise_i8(buf, transmute(num as u8)) }
		num => unsafe { _serialise_i16(buf, num) }
	}
}

pub fn serialise_i32<B: BufferImpl>(buf: &mut B, num: i32) {
	match unsafe { transmute(num) } {
		num @ ..=LARGEST_16 => unsafe { serialise_i16(buf, transmute(num as u16)) }
		num @ ..=LARGEST_24 => unsafe { _serialise_i24(buf, num) }
		num => unsafe { _serialise_i32(buf, num) }
	}
}

pub fn serialise_i64<B: BufferImpl>(buf: &mut B, num: i64) {
	match unsafe { transmute(num) } {
		num @ ..=LARGEST_32 => unsafe { serialise_i32(buf, transmute(num as u32)) }
		num @ ..=LARGEST_40 => unsafe { _serialise_i40(buf, num) }
		num @ ..=LARGEST_48 => unsafe { _serialise_i48(buf, num) }
		num @ ..=LARGEST_56 => unsafe { _serialise_i56(buf, num) }
		num => unsafe { _serialise_i64(buf, num) }
	}
}

pub fn serialise_i128<B: BufferImpl>(buf: &mut B, num: i128) {
	match unsafe { transmute(num) } {
		num @ ..=LARGEST_64 => unsafe { serialise_i64(buf, transmute(num as u64)) }
		num @ ..=LARGEST_72 => unsafe { _serialise_i72(buf, num) }
		num @ ..=LARGEST_80 => unsafe { _serialise_i80(buf, num) }
		num @ ..=LARGEST_88 => unsafe { _serialise_i88(buf, num) }
		num @ ..=LARGEST_96 => unsafe { _serialise_i96(buf, num) }
		num @ ..=LARGEST_104 => unsafe { _serialise_i104(buf, num) }
		num @ ..=LARGEST_112 => unsafe { _serialise_i112(buf, num) }
		num @ ..=LARGEST_120 => unsafe { _serialise_i120(buf, num) }
		num => unsafe { _serialise_i128(buf, num) }
	}
}
