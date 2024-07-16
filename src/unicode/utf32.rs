use super::_internal::{ CodepointUtf32, validate_utf32 };

#[repr(transparent)]
pub struct StrUtf32 {
	inner: [u32]
}

impl StrUtf32 {
	#[inline]
	pub const fn from_utf32(code_units: &[u32]) -> Option<&Self> {
		if validate_utf32(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf32_unchecked(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf32_mut(code_units: &mut [u32]) -> Option<&mut Self> {
		if validate_utf32(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf32_unchecked_mut(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf32_unchecked(utf32: &[u32]) -> &Self {
		// SAFETY: [u32] and Self have same layout
		&*(utf32 as *const [u32] as *const Self)
	}

	#[inline]
	pub unsafe fn from_utf32_unchecked_mut(utf32: &mut [u32]) -> &mut Self {
		// SAFETY: [u32] and Self have same layout
		&mut *(utf32 as *mut [u32] as *mut Self)
	}
}
