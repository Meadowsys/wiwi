//! Internal implementations

use std::{ hint, slice };

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CodepointUtf8 {
	One { value: u8 },
	Two { values: [u8; 2] },
	Three { values: [u8; 3] },
	Four { values: [u8; 4] }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CodepointUtf16 {
	One { value: u16 },
	Two { values: [u16; 2] }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CodepointUtf32 {
	One { value: u32 }
}

/// Returns whether a codepoint is a valid unicode codepoint
#[inline]
pub(super) const fn validate_codepoint(c: u32) -> bool {
	matches!(c, 0x0000..=0xd7ff | 0xe000..=0x10ffff)
}

/// # Safety
///
/// `c` must be a valid unicode codepoint
#[inline]
pub(super) const unsafe fn codepoint_to_utf8_unchecked(c: u32) -> CodepointUtf8 {
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

/// # Safety
///
/// `c` must be a valid unicode codepoint
#[inline]
pub(super) const unsafe fn codepoint_to_utf16_unchecked(c: u32) -> CodepointUtf16 {
	use CodepointUtf16::*;

	if c & 0xffff == c {
		One { value: c as _ }
	} else {
		let c_offset = c - 0x10000;
		let c1 = 0xd800 | (c_offset >> 10) as u16;
		let c2 = 0xdc00 | (c_offset as u16 & 0x3ff);
		Two { values: [c1, c2] }
	}
}

/// # Safety
///
/// `c` must be a valid unicode codepoint
#[inline]
pub(super) const unsafe fn codepoint_to_utf32_unchecked(c: u32) -> CodepointUtf32 {
	CodepointUtf32::One { value: c }
}

/// # Safety
///
/// `utf8` must contain valid data for a UTF-8 codepoint. If it is returned from
/// [`codepoint_to_utf8_unchecked`] (assuming its preconditions were met of
/// course), it is valid.
#[inline]
pub(super) const unsafe fn utf8_to_codepoint_unchecked(utf8: CodepointUtf8) -> u32 {
	match utf8 {
		CodepointUtf8::One { value } => { value as _ }
		CodepointUtf8::Two { values: [c1, c2] } => {
			let c1 = ((c1 & 0x1f) as u32) << 6;
			let c2 = (c2 & 0x3f) as u32;
			c1 | c2
		}
		CodepointUtf8::Three { values: [c1, c2, c3] } => {
			let c1 = ((c1 & 0xf) as u32) << 12;
			let c2 = ((c2 & 0x3f) as u32) << 6;
			let c3 = (c3 & 0x3f) as u32;
			c1 | c2 | c3
		}
		CodepointUtf8::Four { values: [c1, c2, c3, c4] } => {
			let c1 = ((c1 & 0x7) as u32) << 18;
			let c2 = ((c2 & 0x3f) as u32) << 12;
			let c3 = ((c3 & 0x3f) as u32) << 6;
			let c4 = (c4 & 0x3f) as u32;
			c1 | c2 | c3 | c4
		}
	}
}

/// # Safety
///
/// `utf16` must contain valid data for a UTF-16 codepoint. If it is returned from
/// [`codepoint_to_utf16_unchecked`] (assuming its preconditions were met of
/// course), it is valid.
#[inline]
pub(super) const unsafe fn utf16_to_codepoint_unchecked(utf16: CodepointUtf16) -> u32 {
	match utf16 {
		CodepointUtf16::One { value } => { value as _ }
		CodepointUtf16::Two { values: [c1, c2] } => {
			let c1 = ((c1 & 0x3ff) as u32) << 10;
			let c2 = (c2 & 0x3ff) as u32;
			(c1 | c2) + 0x10000
		}
	}
}

/// # Safety
///
/// `utf32` must contain valid data for a UTF-32 codepoint. If it is returned from
/// [`codepoint_to_utf32_unchecked`] (assuming its preconditions were met of
/// course), it is valid.
#[inline]
pub(super) const unsafe fn utf32_to_codepoint_unchecked(utf32: CodepointUtf32) -> u32 {
	match utf32 {
		CodepointUtf32::One { value } => { value }
	}
}

#[inline]
pub(super) const fn validate_utf8(code_units: &[u8]) -> bool {
	// table 3-7

	let len = code_units.len();
	let mut i = 0;

	macro_rules! next_must_match {
		($range:pat) => {
			{
				i += 1;
				if i >= len || !matches!(code_units[i], $range) {
					return false
				}
			}
		}
	}

	while i < len {
		// table 3-7
		match code_units[i] {
			0x00..=0x7f if i % (2 * size_of::<usize>()) == 0 => {
				const _ASSERT_USIZE_ALIGN_LTE_SIZE: () = assert!(size_of::<usize>() >= align_of::<usize>());
				// special ASCII case
				// we attempt to skip ahead quickly for ASCII in usize-sized chunks
				// but try this only every 2-usize chunks

				let remaining = code_units.len() - i;
				if remaining >= (2 * size_of::<usize>()) {
					unsafe {
						const MASK: usize = usize::from_ne_bytes([0x80; size_of::<usize>()]);

						// shift to current i first, then cast to pointer of usize
						// SAFETY: this is sound because the if condition above guarantees
						// sufficient alignment of usize
						let mut ptr = code_units
							.as_ptr()
							.add(i)
							.cast::<usize>();

						// truncating division, so will only return the amount of 2-usize
						// values we can dereference before we go out of bounds. this is
						// always at least one (because of the check above)
						let max = remaining / (2 * size_of::<usize>());

						let mut usize_i = 0;
						while usize_i < max {
							// SAFETY: while loop does not loop more than we can read,
							// so ptr will not dereference out of bounds
							let units = *ptr;
							let units2 = *ptr.add(1);

							// if it isn't valid, this break (which leads to the continue
							// statement after it) would not update the counter, so next
							// loop would start at good spot
							if units & MASK != 0 { break }
							if units2 & MASK != 0 { break }

							usize_i += 2;
							ptr = ptr.add(2);
							i += 2 * size_of::<usize>();
						}

						// if loop did not even run (ie. first usize amount of bytes
						// were not ascii), we need to increment 1, so we only
						// continue/skip forward if usize_i is gt 0 (we looped at least once)
						if usize_i > 0 { continue }
					}
				}

				// else... we fall through as normal (`i` is incremented normally at the end)
				// so we always increment by at least 1, so this does not infinite-loop/deadlock
			}
			0x00..=0x7f => {
				// ASCII (non `2 * size_of::<usize>()` aligned)
			}
			0xc2..=0xdf => {
				next_must_match!(0x80..=0xbf);
			}
			0xe0 => {
				next_must_match!(0xa0..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xe1..=0xec => {
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xed => {
				next_must_match!(0x80..=0x9f);
				next_must_match!(0x80..=0xbf);
			}
			0xee..=0xef => {
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xf0 => {
				next_must_match!(0x90..=0xbf);
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xf1..=0xf3 => {
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xf4 => {
				next_must_match!(0x80..=0x8f);
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			_ => { return false }
		}

		i += 1;
	}

	true
}

#[inline]
pub(super) const fn validate_utf16(code_units: &[u16]) -> bool {
	let len = code_units.len();
	let mut i = 0;

	while i < len {
		match code_units[i] {
			0x0000..=0xd7ff | 0xe000..=0xffff => {
				// BMP
			}
			0xd800..=0xdbff => {
				// leading surrogate code unit
				i += 1;

				if i >= len || !matches!(code_units[i], 0xdc00..=0xdfff) {
					// the next one isn't trailing surrogate code unit
					// isloated leading surrogate (invalid)
					return false
				}
			}
			0xdc00..=0xdfff => {
				// any leading surrogate code units would be checked in previous
				// match arm, and the trailing one would be checked in there as well
				// isloated trailing surrogate (invalid)
				return false
			}
		}

		i += 1;
	}

	true
}

#[inline]
pub(super) const fn validate_utf32(code_units: &[u32]) -> bool {
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

/// # Safety
///
/// The provided code unit slice must be valid UTF-8, and the provided index
/// must be less than or equal to `utf8.len()` (ie. in bounds).
#[inline]
pub(super) const unsafe fn is_char_boundary_utf8_unchecked(utf8: &[u8], i: usize) -> bool {
	if i == 0 || i == utf8.len() { return true }

	matches!(
		*utf8.as_ptr().add(i),
		0x00..=0x7f | 0xc2..=0xdf | 0xe0..=0xef | 0xf0..=0xf4
	)
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-16, and the provided index
/// must be less than or equal to `utf16.len()` (ie. in bounds).
#[inline]
pub(super) const unsafe fn is_char_boundary_utf16_unchecked(utf16: &[u16], i: usize) -> bool {
	if i == 0 || i == utf16.len() { return true }

	// check that it's _not_ a trailing surrogate
	!matches!(*utf16.as_ptr().add(i), 0xdc00..=0xdfff)
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-32, and the provided index
/// must be less than or equal to `utf32.len()` (ie. in bounds).
#[inline]
pub(super) const unsafe fn is_char_boundary_utf32_unchecked(utf32: &[u32], i: usize) -> bool {
	// there are no 2+ unit sequences in UTF-32
	true
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-8, and have a length greater
/// than 0. If both of those preconditions are satisfied, it must mean the slice
/// also has at least one UTF-8 character.
pub(super) const unsafe fn next_codepoint_utf8_unchecked(utf8: &[u8]) -> (u32, &[u8]) {
	debug_assert!(!utf8.is_empty());

	let ptr = utf8.as_ptr();
	let first_cu = *ptr;

	let (cp, consumed) = match first_cu {
		0x00..=0x7f => {
			let cp = CodepointUtf8::One { value: first_cu };
			(cp, 1)
		}
		0xc2..=0xdf => {
			let values = [first_cu, *ptr.add(1)];
			let cp = CodepointUtf8::Two { values };
			(cp, 2)
		}
		0xe0..=0xef => {
			let values = [first_cu, *ptr.add(1), *ptr.add(2)];
			let cp = CodepointUtf8::Three { values };
			(cp, 3)
		}
		0xf0..=0xf4 => {
			let values = [first_cu, *ptr.add(1), *ptr.add(2), *ptr.add(3)];
			let cp = CodepointUtf8::Four { values };
			(cp, 4)
		}
		_ => { hint::unreachable_unchecked() }
	};

	let cp = utf8_to_codepoint_unchecked(cp);
	// TODO: len can be unchecked sub
	let rest = slice::from_raw_parts(ptr.add(consumed), utf8.len() - consumed);
	(cp, rest)
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-16, and have a length greater
/// than 0. If both of those preconditions are satisfied, it must mean the slice
/// also has at least one UTF-16 character.
pub(super) const unsafe fn next_codepoint_utf16_unchecked(utf16: &[u16]) -> (u32, &[u16]) {
	debug_assert!(!utf16.is_empty());

	let ptr = utf16.as_ptr();
	let first_cu = *ptr;

	let (cp, consumed) = match first_cu {
		0x0000..=0xd7ff | 0xe000..=0xffff => {
			let cp = CodepointUtf16::One { value: first_cu };
			(cp, 1)
		}
		0xd800..=0xdbff => {
			let values = [first_cu, *ptr.add(1)];
			let cp = CodepointUtf16::Two { values };
			(cp, 2)
		}
		0xdc00..=0xdfff => { hint::unreachable_unchecked() }
	};

	let cp = utf16_to_codepoint_unchecked(cp);
	// TODO: len can be unchecked sub
	let rest = slice::from_raw_parts(ptr.add(consumed), utf16.len() - consumed);
	(cp, rest)
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-32, and have a length greater
/// than 0. If both of those preconditions are satisfied, it must mean the slice
/// also has at least one UTF-32 character.
pub(super) const unsafe fn next_codepoint_utf32_unchecked(utf32: &[u32]) -> (u32, &[u32]) {
	debug_assert!(!utf32.is_empty());

	let ptr = utf32.as_ptr();
	let first_cu = *ptr;

	let (cp, consumed) = match first_cu {
		0x0000..=0xd7ff | 0xe000..=0x10ffff => {
			let cp = CodepointUtf32::One { value: first_cu };
			(cp, 1)
		}
		_ => { hint::unreachable_unchecked() }
	};

	let cp = utf32_to_codepoint_unchecked(cp);
	// TODO: len can be unchecked sub
	let rest = slice::from_raw_parts(ptr.add(consumed), utf32.len() - consumed);
	(cp, rest)
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
				unsafe { codepoint_to_utf8_unchecked(codepoint) },
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
