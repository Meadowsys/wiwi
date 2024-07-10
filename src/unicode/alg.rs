//! Internal implementations

#[derive(Debug, PartialEq, Eq)]
pub enum CodepointUtf8 {
	One { value: u8 },
	Two { values: [u8; 2] },
	Three { values: [u8; 3] },
	Four { values: [u8; 4] }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CodepointUtf16 {
	One { value: u16 },
	Two { values: [u16; 2] }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CodepointUtf32 {
	One { value: u32 }
}

/// Returns whether a codepoint is a valid unicode codepoint
#[inline]
pub const fn validate_codepoint(c: u32) -> bool {
	!((c > 0xd7ff && c < 0xe000) || c > 0x10ffff)
}

/// # Safety
///
/// `c` must be a valid unicode codepoint
pub const unsafe fn codepoint_to_utf8(c: u32) -> CodepointUtf8 {
	if c & 0x7f == c {
		CodepointUtf8::One { value: c as _ }
	} else if c & 0x7ff == c {
		let c1 = 0xc0 | (c >> 6) as u8;
		let c2 = 0x80 | (c & 0x3f) as u8;
		CodepointUtf8::Two { values: [c1, c2] }
	} else if c & 0xffff == c {
		let c1 = 0xe0 | (c >> 12) as u8;
		let c2 = 0x80 | ((c >> 6) & 0x3f) as u8;
		let c3 = 0x80 | (c & 0x3f) as u8;
		CodepointUtf8::Three { values: [c1, c2, c3] }
	} else {
		let c1 = 0xf0 | (c >> 18) as u8;
		let c2 = 0x80 | ((c >> 12) & 0x3f) as u8;
		let c3 = 0x80 | ((c >> 6) & 0x3f) as u8;
		let c4 = 0x80 | (c & 0x3f) as u8;
		CodepointUtf8::Four { values: [c1, c2, c3, c4] }
	}
}

/// # Safety
///
/// `c` must be a valid unicode codepoint
pub const unsafe fn codepoint_to_utf16(c: u32) -> CodepointUtf16 {
	if c & 0xffff == c {
		CodepointUtf16::One { value: c as _ }
	} else {
		let c_offset = (c - 0x10000) as u16;
		let c1 = 0xd800 | (c_offset >> 10);
		let c2 = 0xdc00 | (c_offset & 0x3ff);
		CodepointUtf16::Two { values: [c1, c2] }
	}
}

/// # Safety
///
/// `c` must be a valid unicode codepoint
#[inline]
pub const unsafe fn codepoint_to_utf32(c: u32) -> CodepointUtf32 {
	CodepointUtf32::One { value: c }
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
				unsafe { super::codepoint_to_utf8(codepoint) },
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
