use super::{ consts, use_ok };
use std::fmt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug)]
pub struct Error {
	expected: &'static str,
	found: &'static str
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
	///
	/// For example:
	///
	/// ```ignore
	/// Error {
	///    expected: "an unsigned integer, up to 24 bits",
	///    found: "a negative integer",
	/// }
	/// ```
	pub mod expected

	// keep order the same as in the `marker` module

	DESC_EXPECTED_MARKER = "a marker"
	DESC_EXPECTED_EOF = "eof"

	// DESC_EXPECTED_SMALLINT_UNSIGNED = "a positive smallint"
	// DESC_EXPECTED_SMALLINT_SIGNED = "a signed smallint"

	DESC_EXPECTED_U8 = "an unsigned integer, 8 bit"
	DESC_EXPECTED_I8 = "a signed integer, 8 bit"
	// ...
	// DESC_EXPECTED_INT_8 = "an integer, up to 8 bits"
	// DESC_EXPECTED_INT_16 = "an integer, up to 16 bits"
	// DESC_EXPECTED_INT_24 = "an integer, up to 24 bits"
	// DESC_EXPECTED_INT_32 = "an integer, up to 32 bits"
	// DESC_EXPECTED_INT_40 = "an integer, up to 40 bits"
	// DESC_EXPECTED_INT_48 = "an integer, up to 48 bits"
	// DESC_EXPECTED_INT_56 = "an integer, up to 56 bits"
	// DESC_EXPECTED_INT_64 = "an integer, up to 64 bits"
	// DESC_EXPECTED_INT_72 = "an integer, up to 72 bits"
	// DESC_EXPECTED_INT_80 = "an integer, up to 80 bits"
	// DESC_EXPECTED_INT_88 = "an integer, up to 88 bits"
	// DESC_EXPECTED_INT_96 = "an integer, up to 96 bits"
	// DESC_EXPECTED_INT_104 = "an integer, up to 104 bits"
	// DESC_EXPECTED_INT_112 = "an integer, up to 112 bits"
	// DESC_EXPECTED_INT_120 = "an integer, up to 120 bits"
	// DESC_EXPECTED_INT_128 = "an integer, up to 128 bits"

	DESC_EXPECTED_BOOL = "a bool"
}

consts! {
	static type &'static str

	/// error messages for the `found` part of errors
	///
	/// For example:
	///
	/// ```ignore
	/// Error {
	///    expected: "an unsigned integer, up to 24 bits",
	///    found: "a negative integer",
	/// }
	/// ```
	pub mod found

	// keep order the same as in the `marker` module

	DESC_FOUND_EOF = "eof"

	DESC_FOUND_SOMETHING_ELSE = "something else"
	DESC_FOUND_TRAILING_BYTES = "trailing bytes"

	// DESC_FOUND_SMALLINT = "a smallint"
	// DESC_FOUND_SMALLINT_NEGATIVE = "a negative smallint"

	// DESC_FOUND_INT_NEGATIVE = "a negative integer"
	// DESC_FOUND_INT_TOO_LARGE_U8 = "an integer that overflows unsigned 8 bits"
	// DESC_FOUND_INT_TOO_LARGE_I8 = "an integer that overflows signed 8 bits"
	// ...
}
