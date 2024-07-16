use super::_internal::{ CodepointUtf16, validate_utf16 };

#[repr(transparent)]
pub struct StrUtf16 {
	inner: [u16]
}

impl StrUtf16 {
	#[inline]
	pub const fn from_utf16(code_units: &[u16]) -> Option<&Self> {
		if validate_utf16(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf16_unchecked(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf16_mut(code_units: &mut [u16]) -> Option<&mut Self> {
		if validate_utf16(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf16_unchecked_mut(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf16_unchecked(utf16: &[u16]) -> &Self {
		// SAFETY: [u16] and Self have same layout
		&*(utf16 as *const [u16] as *const Self)
	}

	#[inline]
	pub unsafe fn from_utf16_unchecked_mut(utf16: &mut [u16]) -> &mut Self {
		// SAFETY: [u16] and Self have same layout
		&mut *(utf16 as *mut [u16] as *mut Self)
	}
}
