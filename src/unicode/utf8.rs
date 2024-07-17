use super::_internal;
use std::mem::transmute;

#[repr(transparent)]
pub struct StrUtf8 {
	inner: [u8]
}

impl StrUtf8 {
	#[inline]
	pub const fn from_utf8(code_units: &[u8]) -> Option<&Self> {
		if _internal::validate_utf8(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf8_unchecked(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf8_mut(code_units: &mut [u8]) -> Option<&mut Self> {
		if _internal::validate_utf8(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf8_unchecked_mut(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf8_unchecked(utf8: &[u8]) -> &Self {
		// SAFETY: [u8] and Self have same layout
		transmute(utf8)
	}

	#[inline]
	pub unsafe fn from_utf8_unchecked_mut(utf8: &mut [u8]) -> &mut Self {
		// SAFETY: [u8] and Self have same layout
		transmute(utf8)
	}

	#[inline]
	pub const fn to_utf8_code_units(&self) -> &[u8] {
		// SAFETY: [u8] and Self have same layout
		unsafe { transmute(self) }
	}

	#[inline]
	pub const fn len_code_units(&self) -> usize {
		self.to_utf8_code_units().len()
	}

	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.len_code_units() == 0
	}

	#[inline]
	pub const fn is_char_boundary(&self, index: usize) -> bool {
		if index <= self.len_code_units() {
			unsafe { _internal::is_char_boundary_utf8_unchecked(self.to_utf8_code_units(), index) }
		} else {
			false
		}
	}
}
