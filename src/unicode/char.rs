use super::_internal;
use std::fmt;

pub struct Char {
	inner: u32
}

impl Char {
	pub unsafe fn from_codepoint_unchecked(c: u32) -> Char {
		Char { inner: c }
	}
}

impl fmt::Display for Char {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use _internal::CodepointUtf8::*;
		use std::str::from_utf8_unchecked;

		// SAFETY: Char is always valid unicode codepoint
		match unsafe { _internal::codepoint_to_utf8_unchecked(self.inner) } {
			// SAFETY: Char is always valid unicode codepoint, so its encoded output is valid UTF-8
			One { value } => { f.write_str(unsafe { from_utf8_unchecked(&[value]) }) }

			// SAFETY: Char is always valid unicode codepoint, so its encoded output is valid UTF-8
			Two { values } => { f.write_str(unsafe { from_utf8_unchecked(&values) }) }

			// SAFETY: Char is always valid unicode codepoint, so its encoded output is valid UTF-8
			Three { values } => { f.write_str(unsafe { from_utf8_unchecked(&values) }) }

			// SAFETY: Char is always valid unicode codepoint, so its encoded output is valid UTF-8
			Four { values } => { f.write_str(unsafe { from_utf8_unchecked(&values) }) }
		}
	}
}
