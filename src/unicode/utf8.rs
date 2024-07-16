use super::_internal::{ CodepointUtf8, validate_utf8 };

#[repr(transparent)]
pub struct StrUtf8 {
	inner: [u8]
}

impl StrUtf8 {
	#[inline]
	pub const fn from_utf8(code_units: &[u8]) -> Option<&Self> {
		if validate_utf8(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf8_unchecked(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf8_mut(code_units: &mut [u8]) -> Option<&mut Self> {
		if validate_utf8(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf8_unchecked_mut(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf8_unchecked(utf8: &[u8]) -> &Self {
		// SAFETY: [u8] and Self have same layout
		&*(utf8 as *const [u8] as *const Self)
	}

	#[inline]
	pub unsafe fn from_utf8_unchecked_mut(utf8: &mut [u8]) -> &mut Self {
		// SAFETY: [u8] and Self have same layout
		&mut *(utf8 as *mut [u8] as *mut Self)
	}

	#[inline]
	pub const fn to_utf8_code_units(&self) -> &[u8] {
		// SAFETY: [u8] and Self have same layout
		unsafe { &*(self as *const Self as *const [u8]) }
	}
}
