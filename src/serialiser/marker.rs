#![deny(dead_code)]

// /// Marker for [`None`], otherwise known as `nil` or `null` in other languages
// pub const MARKER_NONE: u8 = 0;

// markers for numbers: you can get the amount of bytes you need to write by
// doing `marker >> 1` (which is int divide by 2) to get number of bytes needed
// to be written. DO NOT mess with that, it can lead to incorrect results and
// may cause memory unsafety.

/// Marker for unsigned integers that fit in 8 bits, regardless of source size
pub const MARKER_U8: u8 = 2;
/// Marker for signed integers that fit in 8 bits, regardless of source size
pub const MARKER_I8: u8 = 3;

/// Marker for unsigned integers that fit in 16 bits, regardless of source size
pub const MARKER_U16: u8 = 4;
/// Marker for signed integers that fit in 16 bits, regardless of source size
pub const MARKER_I16: u8 = 5;

/// Marker for unsigned integers that fit in 24 bits, regardless of source size
pub const MARKER_U24: u8 = 6;
/// Marker for signed integers that fit in 24 bits, regardless of source size
pub const MARKER_I24: u8 = 7;

/// Marker for unsigned integers that fit in 32 bits, regardless of source size
pub const MARKER_U32: u8 = 8;
/// Marker for signed integers that fit in 32 bits, regardless of source size
pub const MARKER_I32: u8 = 9;

/// Marker for unsigned integers that fit in 40 bits, regardless of source size
pub const MARKER_U40: u8 = 10;
/// Marker for signed integers that fit in 40 bits, regardless of source size
pub const MARKER_I40: u8 = 11;

/// Marker for unsigned integers that fit in 48 bits, regardless of source size
pub const MARKER_U48: u8 = 12;
/// Marker for signed integers that fit in 48 bits, regardless of source size
pub const MARKER_I48: u8 = 13;

/// Marker for unsigned integers that fit in 56 bits, regardless of source size
pub const MARKER_U56: u8 = 14;
/// Marker for signed integers that fit in 56 bits, regardless of source size
pub const MARKER_I56: u8 = 15;

/// Marker for unsigned integers that fit in 64 bits, regardless of source size
pub const MARKER_U64: u8 = 16;
/// Marker for signed integers that fit in 64 bits, regardless of source size
pub const MARKER_I64: u8 = 17;

/// Marker for unsigned integers that fit in 72 bits, regardless of source size
pub const MARKER_U72: u8 = 18;
/// Marker for signed integers that fit in 72 bits, regardless of source size
pub const MARKER_I72: u8 = 19;

/// Marker for unsigned integers that fit in 80 bits, regardless of source size
pub const MARKER_U80: u8 = 20;
/// Marker for signed integers that fit in 80 bits, regardless of source size
pub const MARKER_I80: u8 = 21;

/// Marker for unsigned integers that fit in 88 bits, regardless of source size
pub const MARKER_U88: u8 = 22;
/// Marker for signed integers that fit in 88 bits, regardless of source size
pub const MARKER_I88: u8 = 23;

/// Marker for unsigned integers that fit in 96 bits, regardless of source size
pub const MARKER_U96: u8 = 24;
/// Marker for signed integers that fit in 96 bits, regardless of source size
pub const MARKER_I96: u8 = 25;

/// Marker for unsigned integers that fit in 104 bits, regardless of source size
pub const MARKER_U104: u8 = 26;
/// Marker for signed integers that fit in 104 bits, regardless of source size
pub const MARKER_I104: u8 = 27;

/// Marker for unsigned integers that fit in 112 bits, regardless of source size
pub const MARKER_U112: u8 = 28;
/// Marker for signed integers that fit in 112 bits, regardless of source size
pub const MARKER_I112: u8 = 29;

/// Marker for unsigned integers that fit in 120 bits, regardless of source size
pub const MARKER_U120: u8 = 30;
/// Marker for signed integers that fit in 120 bits, regardless of source size
pub const MARKER_I120: u8 = 31;

/// Marker for unsigned integers that fit in 128 bits, regardless of source size
pub const MARKER_U128: u8 = 32;
/// Marker for signed integers that fit in 128 bits, regardless of source size
pub const MARKER_I128: u8 = 33;

pub const MARKER_HETEROARRAY_SMALL: u8 = 34;
pub const MARKER_HETEROARRAY: u8 = 35;
pub const MARKER_HOMOARRAY_SMALL: u8 = 36;
pub const MARKER_HOMOARRAY: u8 = 37;

// /// Marker for float values that fit in 16 bits (IEEE754-2008 binary16), regardless of source size
// pub const MARKER_F16: u8 = 33;
// /// Marker for float values that fit in 32 bits (IEEE754-2008 binary32), regardless of source size
// pub const MARKER_F32: u8 = 34;
// /// Marker for float values that fit in 64 bits (IEEE754-2008 binary16), regardless of source size
// pub const MARKER_F64: u8 = 35;
// /// reserved (not actually used)
// pub const MARKER_F128: u8 = 36;
// /// reserved (not actually used)
// pub const MARKER_F256: u8 = 37;

// TODO: ?
// /// Marker for a boolean
// pub const MARKER_BOOL: u8 = 38;
// /// Marker for a boolean that also encodes true
// pub const MARKER_BOOL_TRUE: u8 = 39;
// /// Marker for a boolean that also encodes false
// pub const MARKER_BOOL_FALSE: u8 = 40;

/// Use this in a const context, so it can be evaluated to a static string,
/// as well as fail compilation if a description is not available for the
/// given marker (so it can be added to this fn)
#[allow(dead_code)]
#[deny(unreachable_patterns)]
pub const fn const_marker_description<const MARKER: u8>() -> &'static str {
	match MARKER {
		// MARKER_NONE => { "None value" }

		MARKER_U8 => { "unsigned 8-bit integer" }
		MARKER_I8 => { "signed 8-bit integer" }

		MARKER_U16 => { "unsigned 16-bit integer" }
		MARKER_I16 => { "signed 16-bit integer" }

		MARKER_U24 => { "unsigned 24-bit integer" }
		MARKER_I24 => { "signed 24-bit integer" }

		MARKER_U32 => { "unsigned 32-bit integer" }
		MARKER_I32 => { "signed 32-bit integer" }

		MARKER_U40 => { "unsigned 40-bit integer" }
		MARKER_I40 => { "signed 40-bit integer" }

		MARKER_U48 => { "unsigned 48-bit integer" }
		MARKER_I48 => { "signed 48-bit integer" }

		MARKER_U56 => { "unsigned 56-bit integer" }
		MARKER_I56 => { "signed 56-bit integer" }

		MARKER_U64 => { "unsigned 64-bit integer" }
		MARKER_I64 => { "signed 64-bit integer" }

		MARKER_U72 => { "unsigned 72-bit integer" }
		MARKER_I72 => { "signed 72-bit integer" }

		MARKER_U80 => { "unsigned 80-bit integer" }
		MARKER_I80 => { "signed 80-bit integer" }

		MARKER_U88 => { "unsigned 88-bit integer" }
		MARKER_I88 => { "signed 88-bit integer" }

		MARKER_U96 => { "unsigned 96-bit integer" }
		MARKER_I96 => { "signed 96-bit integer" }

		MARKER_U104 => { "unsigned 104-bit integer" }
		MARKER_I104 => { "signed 104-bit integer" }

		MARKER_U112 => { "unsigned 112-bit integer" }
		MARKER_I112 => { "signed 112-bit integer" }

		MARKER_U120 => { "unsigned 120-bit integer" }
		MARKER_I120 => { "signed 120-bit integer" }

		MARKER_U128 => { "unsigned 128-bit integer" }
		MARKER_I128 => { "signed 128-bit integer" }

		MARKER_HETEROARRAY_SMALL => { "heterogenous array" }
		MARKER_HETEROARRAY => { "heterogenous array" }

		MARKER_HOMOARRAY_SMALL => { "homogenous array" }
		MARKER_HOMOARRAY => { "homogenous array" }

		// MARKER_F16 => { "binary16 (half-precision) floating point number" }
		// MARKER_F32 => { "binary32 (single-precision) floating point number" }
		// MARKER_F64 => { "binary64 (double-precision) floating point number" }
		// MARKER_F128 => { "binary128 (quadruple-precision) floating point number" }
		// MARKER_F256 => { "binary256 (octuple-precision) floating point number" }

		// TODO: ?
		// MARKER_BOOL | MARKER_BOOL_TRUE | MARKER_BOOL_FALSE => { "bool" }

		_ => { panic!("unknown marker") }
	}
}
