use super::consts;

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
}

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
pub mod expected {
	use super::*;

	consts! {
		static type &'static str

		// DESC_EXPECTED_SMALLINT_UNSIGNED = "a positive smallint"
		// DESC_EXPECTED_SMALLINT_SIGNED = "a signed smallint"

		// DESC_EXPECTED_U8 = "an unsigned integer, up to 8 bits"
		// DESC_EXPECTED_I8 = "a signed integer, up to 8 bits"
		// ...
	}
}

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
pub mod found {
	use super::*;

	consts! {
		static type &'static str

		// DESC_FOUND_SMALLINT = "a smallint"
		// DESC_FOUND_SMALLINT_NEGATIVE = "a negative smallint"

		// DESC_FOUND_INT_NEGATIVE = "a negative integer"
		// DESC_FOUND_INT_TOO_LARGE_U8 = "an integer that overflows unsigned 8 bits"
		// DESC_FOUND_INT_TOO_LARGE_I8 = "an integer that overflows signed 8 bits"
		// ...
	}
}
