use super::_internal::{ CodepointUtf16, validate_utf16 };
use std::borrow::Cow;
use std::slice;

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

	#[inline]
	pub fn to_utf16_code_units(&self) -> &[u16] {
		// SAFETY: [u16] and Self have same layout
		unsafe { &*(self as *const Self as *const [u16]) }
	}

	#[inline]
	pub fn to_utf16le_bytes(&self) -> Cow<[u8]> {
		// TODO: this can be unchecked mul
		let byte_len = self.inner.len() * 2;

		#[cfg(target_endian = "little")]
		let rv = unsafe {
			let ptr = self.to_utf16_code_units() as *const [u16] as *const u8;
			Cow::Borrowed(slice::from_raw_parts(ptr, byte_len))
		};

		#[cfg(not(target_endian = "little"))]
		let rv = unsafe {
			let mut le_bytes = Vec::with_capacity(byte_len);
			let le_bytes_ptr = le_bytes.as_mut_ptr() as *mut [u8; 2];

			self.to_utf16_code_units()
				.iter()
				.copied()
				.enumerate()
				.for_each(|(i, cu)| le_bytes_ptr.add(i).write(cu.to_le_bytes()));

			Cow::Owned(le_bytes)
		};

		rv
	}

	pub fn to_utf16be_bytes(&self) -> Cow<[u8]> {
		// TODO: this can be unchecked mul
		let byte_len = self.inner.len() * 2;

		#[cfg(target_endian = "big")]
		let rv = unsafe {
			let ptr = self.to_utf16_code_units() as *const [u16] as *const u8;
			Cow::Borrowed(slice::from_raw_parts(ptr, byte_len))
		};

		#[cfg(not(target_endian = "big"))]
		let rv = unsafe {
			let mut be_bytes = Vec::with_capacity(byte_len);
			let be_bytes_ptr = be_bytes.as_mut_ptr() as *mut [u8; 2];

			self.to_utf16_code_units()
				.iter()
				.copied()
				.enumerate()
				.for_each(|(i, cu)| be_bytes_ptr.add(i).write(cu.to_be_bytes()));

			Cow::Owned(be_bytes)
		};

		rv
	}
}
