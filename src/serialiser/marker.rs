use super::error::*;
use ::std::ops::Deref;

pub struct Marker {
	inner: MarkerInner
}

pub enum MarkerInner {
	Number {
		marker: NumberMarker
	},
	Bool {
		marker: BoolMarker
	},
	Null {
		marker: NullMarker
	},
	String {
		marker: StringMarker
	},
	Array {
		marker: ArrayMarker
	},
	Map {
		marker: MapMarker
	}
}

impl Deref for Marker {
	type Target = MarkerInner;
	#[inline]
	fn deref(&self) -> &MarkerInner {
		&self.inner
	}
}

impl Marker {
	fn from_byte(byte: u8) -> Result<Self> {
		match byte {
			num @ 0x00..=0x7f => { NumberMarker::SmallIntPositive { num }.into() }
			num @ 0xc0..=0xff => { NumberMarker::SmallIntNegative { num: num as _ }.into() }

			marker @ 0x80..=0x9f => {
				let byte_count = ((marker & 0b11111) >> 1) + 1;
				if marker & 1 == 0 {
					NumberMarker::Unsigned { byte_count }.into()
				} else {
					NumberMarker::Signed { byte_count }.into()
				}
			}

			0xa0 => { BoolMarker { value: true }.into() }
			0xa1 => { BoolMarker { value: false }.into() }

			// 0xa2 => { NumberMarker::Float16.into() }
			0xa3 => { NumberMarker::Float32.into() }
			0xa4 => { NumberMarker::Float64.into() }
			// 0xa5 => { NumberMarker::Float128.into() }
			// 0xa6 => { NumberMarker::Float256.into() }

			0xa7 => { NullMarker { __private: () }.into() }

			0xa8 => { StringMarker::Variant8.into() }
			0xa9 => { StringMarker::VariantXL.into() }

			0xaa => { ArrayMarker::Variant8.into() }
			0xab => { ArrayMarker::VariantXL.into() }

			0xac => { MapMarker::Variant8.into() }
			0xad => { MapMarker::VariantXL.into() }

			_ => { err("unknown marker") }
		}
	}
	// marker_to_from! {
	// 	u8 {
	// 		(Unsigned8 <-> 0x80)
	// 	}
	// 	u16 {
	// 		(Unsigned16 <-> 0x82)
	// 	}
	// 	u32 {
	// 		(Unsigned24 <-> 0x84)
	// 		(Unsigned32 <-> 0x86)
	// 	}
	// 	u64 {
	// 		(Unsigned40 <-> 0x88)
	// 		(Unsigned48 <-> 0x8a)
	// 		(Unsigned56 <-> 0x8c)
	// 		(Unsigned64 <-> 0x8e)
	// 	}
	// 	u128 {
	// 		(Unsigned72 <-> 0x90)
	// 		(Unsigned80 <-> 0x92)
	// 		(Unsigned88 <-> 0x94)
	// 		(Unsigned96 <-> 0x96)
	// 		(Unsigned104 <-> 0x98)
	// 		(Unsigned112 <-> 0x9a)
	// 		(Unsigned120 <-> 0x9c)
	// 		(Unsigned128 <-> 0x9e)
	// 	}
	// 	i8 {
	// 		(Signed8 <-> 0x81)
	// 	}
	// 	i16 {
	// 		(Signed16 <-> 0x83)
	// 	}
	// 	i32 {
	// 		(Signed24 <-> 0x85)
	// 		(Signed32 <-> 0x87)
	// 	}
	// 	i64 {
	// 		(Signed40 <-> 0x89)
	// 		(Signed48 <-> 0x8b)
	// 		(Signed56 <-> 0x8d)
	// 		(Signed64 <-> 0x8f)
	// 	}
	// 	i128 {
	// 		(Signed72 <-> 0x91)
	// 		(Signed80 <-> 0x93)
	// 		(Signed88 <-> 0x95)
	// 		(Signed96 <-> 0x97)
	// 		(Signed104 <-> 0x99)
	// 		(Signed112 <-> 0x9b)
	// 		(Signed120 <-> 0x9d)
	// 		(Signed128 <-> 0x9f)
	// 	}
	// 	bool {
	// 		(BoolOrTrue <-> 0xa0)
	// 		(False <-> 0xa1)
	// 	}
	// 	float {
	// 		// (Float16 <-> 0xa2)
	// 		(Float32 <-> 0xa3)
	// 		(Float64 <-> 0xa4)
	// 		// (Float128 <-> 0xa5)
	// 		// (Float256 <-> 0xa6)
	// 	}
	// 	null {
	// 		(Null <-> 0xa7)
	// 	}
	// 	string {
	// 		()
	// 	}
	// }

	// // pub fn ensure_unsigned_smallint
	// // pub fn ensure_signed_smallint
}

pub enum NumberMarker {
	SmallIntPositive { num: u8 },
	SmallIntNegative { num: i8 },
	Unsigned { byte_count: u8 },
	Signed { byte_count: u8 },
	// Float16,
	Float32,
	Float64,
	// Float128,
	// Float256
}

pub struct BoolMarker {
	value: bool
}

pub struct NullMarker {
	__private: ()
}

pub enum StringMarker {
	Variant8,
	VariantXL
}

pub enum ArrayMarker {
	Variant8,
	VariantXL
}

pub enum MapMarker {
	Variant8,
	VariantXL
}

macro_rules! marker_from_impls {
	($($container:ident $variant:ident)*) => {
		$(
			impl From<$container> for Marker {
				#[inline]
				fn from(marker: $container) -> Self {
					Self { inner: MarkerInner::$variant { marker } }
				}
			}

			impl<E> From<$container> for Result<Marker, E> {
				#[inline]
				fn from(marker: $container) -> Self {
					Ok(marker.into())
				}
			}
		)*
	}
}

marker_from_impls! {
	NumberMarker Number
	BoolMarker Bool
	NullMarker Null
	StringMarker String
	ArrayMarker Array
	MapMarker Map
}

// pub enum FromByteResult {
// 	Ok(Marker)
// }
//
// pub enum ToByteResult {
// 	Ok(u8)
// }
//
// macro_rules! marker_to_from {
// 	(
// 		u8 { $(($enumval_u8:ident <-> $byte_u8:literal))* }
// 		u16 { $(($enumval_u16:ident <-> $byte_u16:literal))* }
// 		u32 { $(($enumval_u32:ident <-> $byte_u32:literal))* }
// 		u64 { $(($enumval_u64:ident <-> $byte_u64:literal))* }
// 		u128 { $(($enumval_u128:ident <-> $byte_u128:literal))* }
//
// 		i8 { $(($enumval_i8:ident <-> $byte_i8:literal))* }
// 		i16 { $(($enumval_i16:ident <-> $byte_i16:literal))* }
// 		i32 { $(($enumval_i32:ident <-> $byte_i32:literal))* }
// 		i64 { $(($enumval_i64:ident <-> $byte_i64:literal))* }
// 		i128 { $(($enumval_i128:ident <-> $byte_i128:literal))* }
//
// 		bool { $(($enumval_bool:ident <-> $byte_bool:literal))* }
// 		float { $(($enumval_float:ident <-> $byte_float:literal))* }
// 		null { $(($enumval_null:ident <-> $byte_null:literal))* }
// 	) => {
// 		pub fn from_byte(byte: u8) -> Result<FromByteResult> {
// 			#[deny(unreachable_patterns)]
// 			match byte {
// 				byte @ 0x00..=0x7f => { Ok(FromByteResult::Ok(MarkerInner::SmallIntPositive { byte }.into())) }
// 				byte @ 0xc0..=0xff => { Ok(FromByteResult::Ok(MarkerInner::SmallIntNegative { byte }.into())) }
//
// 				$($byte_u8 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_u8.into())) })*
// 				$($byte_u16 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_u16.into())) })*
// 				$($byte_u32 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_u32.into())) })*
// 				$($byte_u64 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_u64.into())) })*
// 				$($byte_u128 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_u128.into())) })*
//
// 				$($byte_i8 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_i8.into())) })*
// 				$($byte_i16 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_i16.into())) })*
// 				$($byte_i32 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_i32.into())) })*
// 				$($byte_i64 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_i64.into())) })*
// 				$($byte_i128 => { Ok(FromByteResult::Ok(MarkerInner::$enumval_i128.into())) })*
//
// 				$($byte_bool => { Ok(FromByteResult::Ok(MarkerInner::$enumval_bool.into())) })*
// 				$($byte_float => { Ok(FromByteResult::Ok(MarkerInner::$enumval_float.into())) })*
// 				$($byte_null => { Ok(FromByteResult::Ok(MarkerInner::$enumval_null.into())) })*

// 				_ => { err("unknown marker") }
// 			}
// 		}
//
// 		pub fn to_byte(&self) -> ToByteResult {
// 			#[deny(unreachable_patterns)]
// 			match self.inner {
// 				MarkerInner::SmallIntPositive { byte } => { ToByteResult::Ok(byte) }
// 				MarkerInner::SmallIntNegative { byte } => { ToByteResult::Ok(byte) }
//
// 				$(MarkerInner::$enumval_u8 => { ToByteResult::Ok($byte_u8) })*
// 				$(MarkerInner::$enumval_u16 => { ToByteResult::Ok($byte_u16) })*
// 				$(MarkerInner::$enumval_u32 => { ToByteResult::Ok($byte_u32) })*
// 				$(MarkerInner::$enumval_u64 => { ToByteResult::Ok($byte_u64) })*
// 				$(MarkerInner::$enumval_u128 => { ToByteResult::Ok($byte_u128) })*
//
// 				$(MarkerInner::$enumval_i8 => { ToByteResult::Ok($byte_i8) })*
// 				$(MarkerInner::$enumval_i16 => { ToByteResult::Ok($byte_i16) })*
// 				$(MarkerInner::$enumval_i32 => { ToByteResult::Ok($byte_i32) })*
// 				$(MarkerInner::$enumval_i64 => { ToByteResult::Ok($byte_i64) })*
// 				$(MarkerInner::$enumval_i128 => { ToByteResult::Ok($byte_i128) })*
//
// 				$(MarkerInner::$enumval_bool => { ToByteResult::Ok($byte_bool) })*
// 				$(MarkerInner::$enumval_float => { ToByteResult::Ok($byte_float) })*
// 				$(MarkerInner::$enumval_null => { ToByteResult::Ok($byte_null) })*
// 			}
// 		}
//
// 		// pub fn ensure_u8_compatible(&self) -> Result<u8> {
// 		// 	#[deny(unreachable_patterns)]
// 		// 	match self {
// 		// 		$(Self::$enumval_u8 => { Ok($byte_u8) })*
// 		// 		_ => { err("expected u8-compatible marker") }
// 		// 	}
// 		// }
// 		//
// 		// pub fn ensure_u16_compatible(&self) -> Result<u8> {
// 		// 	#[deny(unreachable_patterns)]
// 		// 	match self {
// 		// 		$(Self::$enumval_u8 => { Ok($byte_u8) })*
// 		// 		$(Self::$enumval_u16 => { Ok($byte_u16) })*
// 		// 		_ => { err("expected u16-compatible marker") }
// 		// 	}
// 		// }
// 		//
// 		// pub fn ensure_u32_compatible(&self) -> Result<u8> {
// 		// 	#[deny(unreachable_patterns)]
// 		// 	match self {
// 		// 		$(Self::$enumval_u8 => { Ok($byte_u8) })*
// 		// 		$(Self::$enumval_u16 => { Ok($byte_u16) })*
// 		// 		$(Self::$enumval_u32 => { Ok($byte_u32) })*
// 		// 		_ => { err("expected u32-compatible marker") }
// 		// 	}
// 		// }
// 		// etc
// 	}
// }
//
// impl From<MarkerInner> for Marker {
// 	#[inline]
// 	fn from(inner: MarkerInner) -> Self {
// 		Self { inner }
// 	}
// }

// // #![deny(dead_code)]

// /// Marker for [`None`], otherwise known as `nil` or `null` in other languages
// pub const MARKER_NONE: u8 = 0;

// // markers for numbers: you can get the amount of bytes you need to write by
// // doing `marker >> 1` (which is int divide by 2) to get number of bytes needed
// // to be written. DO NOT mess with that, it can lead to incorrect results and
// // may cause memory unsafety.

// /// Marker for unsigned integers that fit in 8 bits, regardless of source size
// pub const MARKER_U8: u8 = 2;
// /// Marker for signed integers that fit in 8 bits, regardless of source size
// pub const MARKER_I8: u8 = 3;

// /// Marker for unsigned integers that fit in 16 bits, regardless of source size
// pub const MARKER_U16: u8 = 4;
// /// Marker for signed integers that fit in 16 bits, regardless of source size
// pub const MARKER_I16: u8 = 5;

// /// Marker for unsigned integers that fit in 24 bits, regardless of source size
// pub const MARKER_U24: u8 = 6;
// /// Marker for signed integers that fit in 24 bits, regardless of source size
// pub const MARKER_I24: u8 = 7;

// /// Marker for unsigned integers that fit in 32 bits, regardless of source size
// pub const MARKER_U32: u8 = 8;
// /// Marker for signed integers that fit in 32 bits, regardless of source size
// pub const MARKER_I32: u8 = 9;

// /// Marker for unsigned integers that fit in 40 bits, regardless of source size
// pub const MARKER_U40: u8 = 10;
// /// Marker for signed integers that fit in 40 bits, regardless of source size
// pub const MARKER_I40: u8 = 11;

// /// Marker for unsigned integers that fit in 48 bits, regardless of source size
// pub const MARKER_U48: u8 = 12;
// /// Marker for signed integers that fit in 48 bits, regardless of source size
// pub const MARKER_I48: u8 = 13;

// /// Marker for unsigned integers that fit in 56 bits, regardless of source size
// pub const MARKER_U56: u8 = 14;
// /// Marker for signed integers that fit in 56 bits, regardless of source size
// pub const MARKER_I56: u8 = 15;

// /// Marker for unsigned integers that fit in 64 bits, regardless of source size
// pub const MARKER_U64: u8 = 16;
// /// Marker for signed integers that fit in 64 bits, regardless of source size
// pub const MARKER_I64: u8 = 17;

// /// Marker for unsigned integers that fit in 72 bits, regardless of source size
// pub const MARKER_U72: u8 = 18;
// /// Marker for signed integers that fit in 72 bits, regardless of source size
// pub const MARKER_I72: u8 = 19;

// /// Marker for unsigned integers that fit in 80 bits, regardless of source size
// pub const MARKER_U80: u8 = 20;
// /// Marker for signed integers that fit in 80 bits, regardless of source size
// pub const MARKER_I80: u8 = 21;

// /// Marker for unsigned integers that fit in 88 bits, regardless of source size
// pub const MARKER_U88: u8 = 22;
// /// Marker for signed integers that fit in 88 bits, regardless of source size
// pub const MARKER_I88: u8 = 23;

// /// Marker for unsigned integers that fit in 96 bits, regardless of source size
// pub const MARKER_U96: u8 = 24;
// /// Marker for signed integers that fit in 96 bits, regardless of source size
// pub const MARKER_I96: u8 = 25;

// /// Marker for unsigned integers that fit in 104 bits, regardless of source size
// pub const MARKER_U104: u8 = 26;
// /// Marker for signed integers that fit in 104 bits, regardless of source size
// pub const MARKER_I104: u8 = 27;

// /// Marker for unsigned integers that fit in 112 bits, regardless of source size
// pub const MARKER_U112: u8 = 28;
// /// Marker for signed integers that fit in 112 bits, regardless of source size
// pub const MARKER_I112: u8 = 29;

// /// Marker for unsigned integers that fit in 120 bits, regardless of source size
// pub const MARKER_U120: u8 = 30;
// /// Marker for signed integers that fit in 120 bits, regardless of source size
// pub const MARKER_I120: u8 = 31;

// /// Marker for unsigned integers that fit in 128 bits, regardless of source size
// pub const MARKER_U128: u8 = 32;
// /// Marker for signed integers that fit in 128 bits, regardless of source size
// pub const MARKER_I128: u8 = 33;

// // // TODO
// /// Marker for float values that fit in 16 bits (IEEE754-2008 binary16), regardless of source size
// pub const MARKER_F16: u8 = 34;
// /// Marker for float values that fit in 32 bits (IEEE754-2008 binary32), regardless of source size
// pub const MARKER_F32: u8 = 35;
// /// Marker for float values that fit in 64 bits (IEEE754-2008 binary16), regardless of source size
// pub const MARKER_F64: u8 = 36;
// // /// reserved (not actually used)
// // pub const MARKER_F128_RESERVED: u8 = 37;
// // /// reserved (not actually used)
// // pub const MARKER_F256_RESERVED: u8 = 38;

// /// Marker for a boolean value
// pub const MARKER_BOOL: u8 = MARKER_BOOL_TRUE;
// /// Marker for the boolean value `true`
// pub const MARKER_BOOL_TRUE: u8 = 39;
// /// Marker for the boolean value `false`
// pub const MARKER_BOOL_FALSE: u8 = 40;

// pub const MARKER_ARRAY_8: u8 = 41;
// pub const MARKER_ARRAY_16: u8 = 42;
// pub const MARKER_ARRAY_XL: u8 = 43;

// pub const MARKER_STRING_8: u8 = 44;
// pub const MARKER_STRING_16: u8 = 45;
// pub const MARKER_STRING_XL: u8 = 46;

// pub const MARKER_OBJECT_8: u8 = 47;
// pub const MARKER_OBJECT_16: u8 = 48;
// pub const MARKER_OBJECT_XL: u8 = 49;

// pub const MARKER_OBJ_ARRAY_KEY_DEDUP: u8 = 50;

// // /// Marker for bools when they are packed into a homogenous array.
// // pub const MARKER_PACKED_BOOLS: u8 = 41;

// // /// Marker for homogenous (single-typed) arrays that have 255 items
// // /// (`u8::MAX`) or less, using a 8-bit int without marker to store the length.
// // pub const MARKER_HOMOARRAY_8: u8 = 34;

// // /// Marker for homogenous (single-typed) arrays that have 65,535 items
// // /// (`u16::MAX`) or less, using a 16-bit int without marker to store the length.
// // pub const MARKER_HOMOARRAY_16: u8 = 35;

// // /// Marker for homogenous (single-typed) arrays that have 4,294,967,295 items
// // /// (about 4.2 billion, or `u32::MAX`) or less, using a 32-bit int
// // /// without marker to store the length.
// // pub const MARKER_HOMOARRAY_32: u8 = 36;

// // /// Marker for homogenous (single-typed) arrays that have...
// // /// 340,282,366,920,938,463,463,374,607,431,768,211,455 items (`u128::MAX`) or
// // /// less, using an arbitrary int type with marker to store the length. Theoretically.
// // /// For now... the more extreme larger lengths are more hypothetical than actually
// // /// useable, especially since rust vecs will abort when reaching
// // /// 9,223,372,036,854,775,807 items (`isize::MAX` on 64bit platforms) stored,
// // /// which is... _probably_ enough :p. If you somehow get here, and you notice,
// // /// perhaps let me know (file an issue?) cause I'll be curious lol
// // pub const MARKER_HOMOARRAY_XL: u8 = 37;

// // /// Use this in a const context, so it can be evaluated to a static string,
// // /// as well as fail compilation if a description is not available for the
// // /// given marker (so it can be added to this fn)
// // #[allow(dead_code)]
// // #[deny(unreachable_patterns)]
// // pub const fn const_marker_description<const MARKER: u8>() -> &'static str {
// // 	match MARKER {
// // 		// MARKER_NONE => { "None value" }

// // 		MARKER_U8 => { "unsigned 8-bit integer" }
// // 		MARKER_I8 => { "signed 8-bit integer" }

// // 		MARKER_U16 => { "unsigned 16-bit integer" }
// // 		MARKER_I16 => { "signed 16-bit integer" }

// // 		MARKER_U24 => { "unsigned 24-bit integer" }
// // 		MARKER_I24 => { "signed 24-bit integer" }

// // 		MARKER_U32 => { "unsigned 32-bit integer" }
// // 		MARKER_I32 => { "signed 32-bit integer" }

// // 		MARKER_U40 => { "unsigned 40-bit integer" }
// // 		MARKER_I40 => { "signed 40-bit integer" }

// // 		MARKER_U48 => { "unsigned 48-bit integer" }
// // 		MARKER_I48 => { "signed 48-bit integer" }

// // 		MARKER_U56 => { "unsigned 56-bit integer" }
// // 		MARKER_I56 => { "signed 56-bit integer" }

// // 		MARKER_U64 => { "unsigned 64-bit integer" }
// // 		MARKER_I64 => { "signed 64-bit integer" }

// // 		MARKER_U72 => { "unsigned 72-bit integer" }
// // 		MARKER_I72 => { "signed 72-bit integer" }

// // 		MARKER_U80 => { "unsigned 80-bit integer" }
// // 		MARKER_I80 => { "signed 80-bit integer" }

// // 		MARKER_U88 => { "unsigned 88-bit integer" }
// // 		MARKER_I88 => { "signed 88-bit integer" }

// // 		MARKER_U96 => { "unsigned 96-bit integer" }
// // 		MARKER_I96 => { "signed 96-bit integer" }

// // 		MARKER_U104 => { "unsigned 104-bit integer" }
// // 		MARKER_I104 => { "signed 104-bit integer" }

// // 		MARKER_U112 => { "unsigned 112-bit integer" }
// // 		MARKER_I112 => { "signed 112-bit integer" }

// // 		MARKER_U120 => { "unsigned 120-bit integer" }
// // 		MARKER_I120 => { "signed 120-bit integer" }

// // 		MARKER_U128 => { "unsigned 128-bit integer" }
// // 		MARKER_I128 => { "signed 128-bit integer" }

// // 		MARKER_BOOL_TRUE | MARKER_BOOL_FALSE => { "bool" }

// // 		MARKER_F16 => { "binary16 (half-precision) floating point number" }
// // 		MARKER_F32 => { "binary32 (single-precision) floating point number" }
// // 		MARKER_F64 => { "binary64 (double-precision) floating point number" }
// // 		MARKER_F128 => { "binary128 (quadruple-precision) floating point number" }
// // 		MARKER_F256 => { "binary256 (octuple-precision) floating point number" }

// // 		// MARKER_HOMOARRAY_8 => { "homogenous array" }
// // 		// MARKER_HOMOARRAY_16 => { "homogenous array" }
// // 		// MARKER_HOMOARRAY_32 => { "homogenous array" }
// // 		// MARKER_HOMOARRAY_XL => { "homogenous array" }

// // 		_ => { panic!("unknown marker") }
// // 	}
// // }
