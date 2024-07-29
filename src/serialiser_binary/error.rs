use super::{ consts, use_ok };
use std::fmt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug)]
pub struct Error {
	pub expected: &'static str,
	pub found: &'static str
}

impl Error {
	#[inline(always)]
	pub fn wrap<T>(self) -> Result<T> {
		Err(self)
	}

	#[inline(always)]
	pub fn wrap_foreign<T, E>(self) -> Result<T, E>
	where
		Error: Into<E>
	{
		Err(self.into())
	}

	#[inline(always)]
	pub fn expected(mut self, expected: &'static str) -> Self {
		self.expected = expected;
		self
	}

	#[inline(always)]
	pub fn found(mut self, found: &'static str) -> Self {
		self.found = found;
		self
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use_ok!(f.write_str("expected "));
		use_ok!(f.write_str(self.expected));
		use_ok!(f.write_str(", found "));
		use_ok!(f.write_str(self.found));

		Ok(())
	}
}

impl std::error::Error for Error {}

#[repr(transparent)]
pub struct ErrorExpected {
	expected: &'static str
}

#[inline(always)]
pub fn expected(expected: &'static str) -> ErrorExpected {
	ErrorExpected { expected }
}

impl ErrorExpected {
	#[inline(always)]
	pub fn found(self, found: &'static str) -> Error {
		let ErrorExpected { expected } = self;
		Error { expected, found }
	}

	#[inline(always)]
	pub fn found_something_else(self) -> Error {
		let ErrorExpected { expected } = self;
		let found = found::DESC_FOUND_SOMETHING_ELSE;
		Error { expected, found }
	}

	#[inline(always)]
	pub fn found_eof(self) -> Error {
		let ErrorExpected { expected } = self;
		let found = found::DESC_FOUND_EOF;
		Error { expected, found }
	}

	#[inline(always)]
	pub fn wrap<T>(self) -> Result<T, Self> {
		Err(self)
	}
}

#[repr(transparent)]
pub struct ErrorFound {
	found: &'static str
}

#[inline(always)]
pub fn found(found: &'static str) -> ErrorFound {
	ErrorFound { found }
}

#[inline(always)]
pub fn found_something_else() -> ErrorFound {
	ErrorFound { found: found::DESC_FOUND_SOMETHING_ELSE }
}

#[inline(always)]
pub fn found_eof() -> ErrorFound {
	ErrorFound { found: found::DESC_FOUND_EOF }
}

impl ErrorFound {
	#[inline(always)]
	pub fn expected(self, expected: &'static str) -> Error {
		let ErrorFound { found } = self;
		Error { expected, found }
	}

	#[inline(always)]
	pub fn wrap<T>(self) -> Result<T, Self> {
		Err(self)
	}
}

consts! {
	static type &'static str

	/// error messages for the `expected` part of errors
	pub mod expected

	// keep order the same as in the `marker` module

	DESC_EXPECTED_MARKER = "a marker"
	DESC_EXPECTED_EOF = "eof"

	// DESC_EXPECTED_SMALLINT_UNSIGNED = "a positive smallint"
	// DESC_EXPECTED_SMALLINT_SIGNED = "a signed smallint"

	DESC_EXPECTED_U8 = "an unsigned integer, 8 bit"
	DESC_EXPECTED_I8 = "a signed integer, 8 bit"

	DESC_EXPECTED_U16 = "an unsigned integer, 16 bit"
	DESC_EXPECTED_I16 = "a signed integer, 16 bit"

	DESC_EXPECTED_U24 = "an unsigned integer, 24 bit"
	DESC_EXPECTED_I24 = "a signed integer, 24 bit"

	DESC_EXPECTED_U32 = "an unsigned integer, 32 bit"
	DESC_EXPECTED_I32 = "a signed integer, 32 bit"

	DESC_EXPECTED_U40 = "an unsigned integer, 40 bit"
	DESC_EXPECTED_I40 = "a signed integer, 40 bit"

	DESC_EXPECTED_U48 = "an unsigned integer, 48 bit"
	DESC_EXPECTED_I48 = "a signed integer, 48 bit"

	DESC_EXPECTED_U56 = "an unsigned integer, 56 bit"
	DESC_EXPECTED_I56 = "a signed integer, 56 bit"

	DESC_EXPECTED_U64 = "an unsigned integer, 64 bit"
	DESC_EXPECTED_I64 = "a signed integer, 64 bit"

	DESC_EXPECTED_U72 = "an unsigned integer, 72 bit"
	DESC_EXPECTED_I72 = "a signed integer, 72 bit"

	DESC_EXPECTED_U80 = "an unsigned integer, 80 bit"
	DESC_EXPECTED_I80 = "a signed integer, 80 bit"

	DESC_EXPECTED_U88 = "an unsigned integer, 88 bit"
	DESC_EXPECTED_I88 = "a signed integer, 88 bit"

	DESC_EXPECTED_U96 = "an unsigned integer, 96 bit"
	DESC_EXPECTED_I96 = "a signed integer, 96 bit"

	DESC_EXPECTED_U104 = "an unsigned integer, 104 bit"
	DESC_EXPECTED_I104 = "a signed integer, 104 bit"

	DESC_EXPECTED_U112 = "an unsigned integer, 112 bit"
	DESC_EXPECTED_I112 = "a signed integer, 112 bit"

	DESC_EXPECTED_U120 = "an unsigned integer, 120 bit"
	DESC_EXPECTED_I120 = "a signed integer, 120 bit"

	DESC_EXPECTED_U128 = "an unsigned integer, 128 bit"
	DESC_EXPECTED_I128 = "a signed integer, 128 bit"

	#[cfg(target_pointer_width = "64")]
	DESC_EXPECTED_USIZE = "an unsigned integer, word sized (64 bit)"
	#[cfg(target_pointer_width = "32")]
	DESC_EXPECTED_USIZE = "an unsigned integer, word sized (32 bit)"
	#[cfg(target_pointer_width = "16")]
	DESC_EXPECTED_USIZE = "an unsigned integer, word sized (16 bit)"

	#[cfg(target_pointer_width = "64")]
	DESC_EXPECTED_ISIZE = "a signed integer, word sized (64 bit)"
	#[cfg(target_pointer_width = "32")]
	DESC_EXPECTED_ISIZE = "a signed integer, word sized (32 bit)"
	#[cfg(target_pointer_width = "16")]
	DESC_EXPECTED_ISIZE = "a signed integer, word sized (16 bit)"

	DESC_EXPECTED_BOOL = "a bool"

	DESC_EXPECTED_F32 = "a floating point number, 32 bit"
	DESC_EXPECTED_F64 = "a floating point number, 64 bit"

	DESC_EXPECTED_STR = "an utf-8 encoded string"
	DESC_EXPECTED_ARRAY = "an array"
	DESC_EXPECTED_MAP = "a map"
}

consts! {
	static type &'static str

	/// error messages for the `found` part of errors
	pub mod found

	// keep order the same as in the `marker` module

	DESC_FOUND_EOF = "eof"

	DESC_FOUND_SOMETHING_ELSE = "something else"
	DESC_FOUND_TRAILING_BYTES = "trailing bytes"

	DESC_FOUND_OVERFLOWING_INT = "an overflowing integer"

	// DESC_FOUND_SMALLINT = "a smallint"
	// DESC_FOUND_SMALLINT_NEGATIVE = "a negative smallint"

	// DESC_FOUND_INT_NEGATIVE = "a negative integer"
	// DESC_FOUND_INT_TOO_LARGE_U8 = "an integer that overflows unsigned 8 bits"
	// DESC_FOUND_INT_TOO_LARGE_I8 = "an integer that overflows signed 8 bits"
	// ...

	DESC_FOUND_INVALID_UTF_8 = "invalid utf-8"
}
