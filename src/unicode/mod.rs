// TODO: implement and conform to unicode 15.1. If other versions come
// out in the meantime, finish implementing 15.1 first, I think.
//
// https://www.unicode.org/versions/Unicode15.1.0/

pub mod utf8;
pub mod utf16;
pub mod utf32;

#[derive(Clone, Copy)]
pub struct Char {
	inner: u32
}

impl Char {
	#[inline]
	pub const fn from_std_char(c: std::primitive::char) -> Self {
		// std's char has same invariants that we do
		debug_assert!(Self::try_from_u32(c as _).is_some());

		Self { inner: c as _ }
	}

	#[inline]
	pub const fn try_from_u32(c: u32) -> Option<Self> {
		if (c > 0xd7ff && c < 0xe000) || c > 0x10ffff {
			None
		} else {
			Some(Self { inner: c })
		}
	}

	#[inline]
	pub const unsafe fn from_u32_unchecked(c: u32) -> Self {
		debug_assert!(Self::try_from_u32(c).is_some(), "invalid unicode code point");
		Self { inner: c }
	}

	#[inline]
	pub const fn to_u32(self) -> u32 {
		self.inner
	}

	pub const fn encode_utf8(self) -> CharUtf8 {
		let c = self.to_u32();

		if c & 0x7f == c {
			CharUtf8::One { value: c as _ }
		} else if c & 0x7ff == c {
			let c1 = 0xc0 | (c >> 6) as u8;
			let c2 = 0x80 | (c & 0x3f) as u8;
			CharUtf8::Two { values: [c1, c2] }
		} else if c & 0xffff == c {
			let c1 = 0xe0 | (c >> 12) as u8;
			let c2 = 0x80 | ((c >> 6) & 0x3f) as u8;
			let c3 = 0x80 | (c & 0x3f) as u8;
			CharUtf8::Three { values: [c1, c2, c3] }
		} else {
			let c1 = 0xf0 | (c >> 18) as u8;
			let c2 = 0x80 | ((c >> 12) & 0x3f) as u8;
			let c3 = 0x80 | ((c >> 6) & 0x3f) as u8;
			let c4 = 0x80 | (c & 0x3f) as u8;
			CharUtf8::Four { values: [c1, c2, c3, c4] }
		}
	}

	pub const fn encode_utf16(self) -> CharUtf16 {
		let c = self.to_u32();
		if c & 0xffff == c {
			CharUtf16::One { value: c as _ }
		} else {
			let c_offset = (c - 0x10000) as u16;
			let c1 = 0xd800 | (c_offset >> 10);
			let c2 = 0xdc00 | (c_offset & 0x3ff);
			CharUtf16::Two { values: [c1, c2] }
		}
	}

	pub const fn encode_utf32(self) -> CharUtf32 {
		CharUtf32::One { value: self.to_u32() }
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum CharUtf8 {
	One { value: u8 },
	Two { values: [u8; 2] },
	Three { values: [u8; 3] },
	Four { values: [u8; 4] }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CharUtf16 {
	One { value: u16 },
	Two { values: [u16; 2] }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CharUtf32 {
	One { value: u32 }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encodes_utf8() {
		// testing endpoints of the code point ranges in
		// table 3-7

		trait MkCharUtf8 {
			fn make_char_utf8(self) -> CharUtf8;
		}

		impl MkCharUtf8 for u8 {
			fn make_char_utf8(self) -> CharUtf8 {
				CharUtf8::One { value: self }
			}
		}
		impl MkCharUtf8 for [u8; 2] {
			fn make_char_utf8(self) -> CharUtf8 {
				CharUtf8::Two { values: self }
			}
		}

		impl MkCharUtf8 for [u8; 3] {
			fn make_char_utf8(self) -> CharUtf8 {
				CharUtf8::Three { values: self }
			}
		}

		impl MkCharUtf8 for [u8; 4] {
			fn make_char_utf8(self) -> CharUtf8 {
				CharUtf8::Four { values: self }
			}
		}

		fn check<T: MkCharUtf8>(codepoint: u32, expected: T) {
			assert_eq!(
				Char::try_from_u32(codepoint).unwrap().encode_utf8(),
				expected.make_char_utf8()
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
