use super::_internal;
use std::mem::transmute;

#[repr(transparent)]
pub struct StrUtf32 {
	inner: [u32]
}

pub struct StringUtf32 {
	inner: Vec<u32>
}

impl StrUtf32 {
	#[inline]
	pub const fn from_utf32(code_units: &[u32]) -> Option<&Self> {
		if _internal::validate_utf32(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf32_unchecked(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf32_mut(code_units: &mut [u32]) -> Option<&mut Self> {
		if _internal::validate_utf32(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf32_unchecked_mut(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf32_unchecked(utf32: &[u32]) -> &Self {
		// SAFETY: [u32] and Self have same layout
		transmute(utf32)
	}

	#[inline]
	pub unsafe fn from_utf32_unchecked_mut(utf32: &mut [u32]) -> &mut Self {
		// SAFETY: [u32] and Self have same layout
		transmute(utf32)
	}

	pub const fn to_utf32_code_units(&self) -> &[u32] {
		unsafe { transmute(self) }
	}

	#[inline]
	pub const fn len_code_units(&self) -> usize {
		self.to_utf32_code_units().len()
	}

	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.len_code_units() == 0
	}

	#[inline]
	pub const fn is_char_boundary(&self, index: usize) -> bool {
		if index <= self.len_code_units() {
			unsafe { _internal::is_char_boundary_utf32_unchecked(self.to_utf32_code_units(), index) }
		} else {
			false
		}
	}
}

impl StringUtf32 {
	#[inline]
	pub const fn new() -> Self {
		Self { inner: Vec::new() }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		Self { inner: Vec::with_capacity(capacity) }
	}
}
