use super::{ Char, CharUtf32, validate_codepoint };
use std::mem::transmute;

#[repr(transparent)]
pub struct Str {
	inner: [u32]
}

pub struct String {
	inner: Vec<u32>
}

impl Str {
	#[inline]
	pub const fn from_utf32(utf32: &[u32]) -> Option<&Str> {
		if run_utf32_validation(utf32) {
			// SAFETY: just validated
			Some(unsafe { Self::from_utf32_unchecked(utf32) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf32_mut(utf32: &mut [u32]) -> Option<&mut Str> {
		if run_utf32_validation(utf32) {
			// SAFETY: just validated
			Some(unsafe { Self::from_utf32_unchecked_mut(utf32) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf32_unchecked(utf32: &[u32]) -> &Str {
		transmute(utf32)
	}

	#[inline]
	pub unsafe fn from_utf32_unchecked_mut(utf32: &mut [u32]) -> &mut Str {
		transmute(utf32)
	}
}

impl String {
	#[inline]
	pub fn from_utf32(utf32: Vec<u32>) -> Result<Self, Vec<u32>> {
		if run_utf32_validation(&utf32) {
			// SAFETY: just validated
			Ok(unsafe { Self::from_utf32_unchecked(utf32) })
		} else {
			Err(utf32)
		}
	}

	#[inline]
	pub unsafe fn from_utf32_unchecked(utf32: Vec<u32>) -> String {
		String { inner: utf32 }
	}

	pub fn push_char(&mut self, c: Char) {
		match c.encode_utf32() {
			CharUtf32::One { value } => { self.inner.push(value) }
		}
	}
}

#[inline]
pub const fn run_utf32_validation(utf32: &[u32]) -> bool {
	let len = utf32.len();
	let mut i = 0;

	while i < len {
		let c = utf32[i];
		// every utf32 code unit is just a codepoint
		if !validate_codepoint(c) { return false }
		i += 1;
	}

	true
}
