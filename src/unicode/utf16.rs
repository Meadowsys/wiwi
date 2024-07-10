use super::{ Char, CharUtf16 };
use std::mem::transmute;

#[repr(transparent)]
pub struct Str {
	inner: [u16]
}

pub struct String {
	inner: Vec<u16>
}

impl Str {
	#[inline]
	pub const fn from_utf16(utf16: &[u16]) -> Option<&Str> {
		if run_utf16_validation(utf16) {
			// SAFETY: just validated
			Some(unsafe { Self::from_utf16_unchecked(utf16) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf16_mut(utf16: &mut [u16]) -> Option<&mut Str> {
		if run_utf16_validation(utf16) {
			// SAFETY: just validated
			Some(unsafe { Self::from_utf16_unchecked_mut(utf16) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf16_unchecked(utf16: &[u16]) -> &Str {
		transmute(utf16)
	}

	#[inline]
	pub unsafe fn from_utf16_unchecked_mut(utf16: &mut [u16]) -> &mut Str {
		transmute(utf16)
	}
}

impl String {
	#[inline]
	pub fn from_utf16(utf16: Vec<u16>) -> Result<Self, Vec<u16>> {
		if run_utf16_validation(&utf16) {
			// SAFETY: just validated
			Ok(unsafe { Self::from_utf16_unchecked(utf16) })
		} else {
			Err(utf16)
		}
	}

	#[inline]
	pub unsafe fn from_utf16_unchecked(utf16: Vec<u16>) -> String {
		String { inner: utf16 }
	}

	pub fn push_char(&mut self, c: Char) {
		match c.encode_utf16() {
			CharUtf16::One { value } => { self.inner.push(value) }
			CharUtf16::Two { values } => { self.inner.extend_from_slice(&values) }
		}
	}
}

pub const fn run_utf16_validation(utf16: &[u16]) -> bool {
	let len = utf16.len();
	let mut i = 0;

	while i < len {
		match utf16[i] {
			0..=0xd7ff | 0xe000..=0xffff => { i += 1 }
			0xd800..=0xdbff => {
				let next = i + 1;
				if next >= len || !matches!(utf16[next], 0xdc00..=0xdfff) {
					return false
				}
				i += 2;
			}
			_ => { return false }
		}
	}

	true
}
