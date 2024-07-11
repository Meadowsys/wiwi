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

#[inline]
pub const fn validate_utf8(code_units: &[u8]) -> bool {
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

	'outer: while i < len {
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
						if usize_i > 0 { continue 'outer }
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
pub const fn validate_utf16(code_units: &[u16]) -> bool {
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
