use super::_internal;
use std::fmt;

pub struct Char {
	inner: u32
}

impl Char {
	#[inline]
	pub unsafe fn from_codepoint_unchecked(c: u32) -> Char {
		Char { inner: c }
	}
}

impl fmt::Display for Char {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// SAFETY: Char is always valid unicode codepoint
		let cp = unsafe { _internal::codepoint_to_utf8_unchecked(self.inner) };

		let utf8: &[u8] = match &cp {
			_internal::CodepointUtf8::One { values } => { values }
			_internal::CodepointUtf8::Two { values } => { values }
			_internal::CodepointUtf8::Three { values } => { values }
			_internal::CodepointUtf8::Four { values } => { values }
		};

		// SAFETY: Char is always valid unicode codepoint, so its encoded output is valid UTF-8
		f.write_str(unsafe { std::str::from_utf8_unchecked(utf8) })
	}
}
