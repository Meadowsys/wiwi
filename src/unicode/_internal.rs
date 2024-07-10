//! Internal implementations

/// Returns whether a codepoint is a valid unicode codepoint
#[inline]
pub(super) const fn validate_codepoint(c: u32) -> bool {
	!((c > 0xd7ff && c < 0xe000) || c > 0x10ffff)
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CodepointUtf8 {
	One { value: u8 },
	Two { values: [u8; 2] },
	Three { values: [u8; 3] },
	Four { values: [u8; 4] }
}

impl CodepointUtf8 {
	/// # Safety
	///
	/// `c` must be a valid unicode codepoint
	pub(super) const unsafe fn from_codepoint_unchecked(c: u32) -> Self {
		use CodepointUtf8::*;

		if c & 0x7f == c {
			One { value: c as _ }
		} else if c & 0x7ff == c {
			let c1 = 0xc0 | (c >> 6) as u8;
			let c2 = 0x80 | (c & 0x3f) as u8;
			Two { values: [c1, c2] }
		} else if c & 0xffff == c {
			let c1 = 0xe0 | (c >> 12) as u8;
			let c2 = 0x80 | ((c >> 6) & 0x3f) as u8;
			let c3 = 0x80 | (c & 0x3f) as u8;
			Three { values: [c1, c2, c3] }
		} else {
			let c1 = 0xf0 | (c >> 18) as u8;
			let c2 = 0x80 | ((c >> 12) & 0x3f) as u8;
			let c3 = 0x80 | ((c >> 6) & 0x3f) as u8;
			let c4 = 0x80 | (c & 0x3f) as u8;
			Four { values: [c1, c2, c3, c4] }
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CodepointUtf16 {
	One { value: u16 },
	Two { values: [u16; 2] }
}

impl CodepointUtf16 {
	/// # Safety
	///
	/// `c` must be a valid unicode codepoint
	pub(super) const unsafe fn from_codepoint_unchecked(c: u32) -> Self {
		use CodepointUtf16::*;

		if c & 0xffff == c {
			One { value: c as _ }
		} else {
			let c_offset = (c - 0x10000) as u16;
			let c1 = 0xd800 | (c_offset >> 10);
			let c2 = 0xdc00 | (c_offset & 0x3ff);
			Two { values: [c1, c2] }
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CodepointUtf32 {
	One { value: u32 }
}

impl CodepointUtf32 {
	/// # Safety
	///
	/// `c` must be a valid unicode codepoint
	#[inline]
	pub(super) const unsafe fn from_codepoint_unchecked(c: u32) -> CodepointUtf32 {
		CodepointUtf32::One { value: c }
	}
}

pub const fn validate_utf16(code_units: &[u16]) -> bool {
	let len = code_units.len();
	let mut i = 0;

	while i < len {
		match code_units[i] {
			0..=0xd7ff | 0xe000..=0xffff => {
				// BMP
				i += 1;
			}
			0xd800..=0xdbff => {
				// leading surrogate code unit
				let next = i + 1;
				if next >= len || !matches!(code_units[next], 0xdc00..=0xdfff) {
					// the next one isn't trailing surrogate code unit
					// isloated leading surrogate (invalid)
					return false
				}
				i += 2;
			}
			0xdc00..=0xdfff => {
				// any leading surrogate code units would be checked in previous
				// match arm, and the trailing one would be checked in there as well
				// isloated trailing surrogate (invalid)
				return false
			}
		}
	}

	true
}

#[inline]
pub const fn validate_utf32(code_units: &[u32]) -> bool {
	let len = code_units.len();
	let mut i = 0;

	while i < len {
		let c = code_units[i];
		// every utf32 code unit is just a codepoint
		if !validate_codepoint(c) { return false }
		i += 1;
	}

	true
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn codepoint_to_utf8() {
		// testing endpoints of the code point ranges in
		// table 3-7

		trait MkCodepointUtf8 {
			fn make_codepoint_utf8(self) -> CodepointUtf8;
		}

		impl MkCodepointUtf8 for u8 {
			fn make_codepoint_utf8(self) -> CodepointUtf8 {
				CodepointUtf8::One { value: self }
			}
		}
		impl MkCodepointUtf8 for [u8; 2] {
			fn make_codepoint_utf8(self) -> CodepointUtf8 {
				CodepointUtf8::Two { values: self }
			}
		}

		impl MkCodepointUtf8 for [u8; 3] {
			fn make_codepoint_utf8(self) -> CodepointUtf8 {
				CodepointUtf8::Three { values: self }
			}
		}

		impl MkCodepointUtf8 for [u8; 4] {
			fn make_codepoint_utf8(self) -> CodepointUtf8 {
				CodepointUtf8::Four { values: self }
			}
		}

		fn check<T: MkCodepointUtf8>(codepoint: u32, expected: T) {
			assert!(validate_codepoint(codepoint));
			assert_eq!(
				// SAFETY: just asserted codepoint is valid above
				unsafe { CodepointUtf8::from_codepoint_unchecked(codepoint) },
				expected.make_codepoint_utf8()
			);
		}

		check(0x0000, 0x00);
		check(0x007f, 0x7f);
		check(0x0080, [0xc2, 0x80]);
		check(0x07ff, [0xdf, 0xbf]);
		check(0x0800, [0xe0, 0xa0, 0x80]);
		check(0x0fff, [0xe0, 0xbf, 0xbf]);
		check(0x1000, [0xe1, 0x80, 0x80]);
		check(0xcfff, [0xec, 0xbf, 0xbf]);
		check(0xd000, [0xed, 0x80, 0x80]);
		check(0xd7ff, [0xed, 0x9f, 0xbf]);
		check(0xe000, [0xee, 0x80, 0x80]);
		check(0xffff, [0xef, 0xbf, 0xbf]);
		check(0x10000, [0xf0, 0x90, 0x80, 0x80]);
		check(0x3ffff, [0xf0, 0xbf, 0xbf, 0xbf]);
		check(0x40000, [0xf1, 0x80, 0x80, 0x80]);
		check(0xfffff, [0xf3, 0xbf, 0xbf, 0xbf]);
		check(0x100000, [0xf4, 0x80, 0x80, 0x80]);
		check(0x10ffff, [0xf4, 0x8f, 0xbf, 0xbf]);
	}
}
