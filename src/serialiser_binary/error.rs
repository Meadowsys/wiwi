use super::{ consts, use_ok };
use std::fmt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug)]
pub struct Error {
	expected: &'static str,
	found: &'static str
}

#[repr(transparent)]
pub struct ErrorHalf {
	expected: &'static str
}

#[inline(always)]
pub(crate) fn expected(expected: &'static str) -> ErrorHalf {
	ErrorHalf { expected }
}

impl ErrorHalf {
	#[inline(always)]
	pub(crate) fn found(self, found: &'static str) -> Error {
		let ErrorHalf { expected } = self;
		Error { expected, found }
	}

	#[inline(always)]
	pub(crate) fn found_something(self) -> Error {
		let ErrorHalf { expected } = self;
		let found = found::DESC_FOUND_SOMETHING_ELSE;
		Error { expected, found }
	}

	#[inline(always)]
	pub(crate) fn found_nothing(self) -> Error {
		let ErrorHalf { expected } = self;
		let found = found::DESC_FOUND_NOTHING;
		Error { expected, found }
	}
}

impl std::error::Error for Error {}

impl Error {
	#[inline(always)]
	pub(crate) fn wrap_in_err<T>(self) -> Result<T> {
		Err(self)
	}

	#[inline(always)]
	pub(crate) fn wrap_in_foreign_err<T, E>(self) -> Result<T, E>
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

	DESC_EXPECTED_EOF = "eof"

	// DESC_EXPECTED_SMALLINT_UNSIGNED = "a positive smallint"
	// DESC_EXPECTED_SMALLINT_SIGNED = "a signed smallint"

	// DESC_EXPECTED_U8 = "an unsigned integer, up to 8 bits"
	// DESC_EXPECTED_I8 = "a signed integer, up to 8 bits"
	// ...

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

	DESC_FOUND_SOMETHING_ELSE = "something else"
	DESC_FOUND_NOTHING = "nothing"
	DESC_FOUND_TRAILING_BYTES = "trailing bytes"

	// DESC_FOUND_SMALLINT = "a smallint"
	// DESC_FOUND_SMALLINT_NEGATIVE = "a negative smallint"

	// DESC_FOUND_INT_NEGATIVE = "a negative integer"
	// DESC_FOUND_INT_TOO_LARGE_U8 = "an integer that overflows unsigned 8 bits"
	// DESC_FOUND_INT_TOO_LARGE_I8 = "an integer that overflows signed 8 bits"
	// ...
}
