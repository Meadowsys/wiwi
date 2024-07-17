use super::_internal;
use std::borrow::Cow;
use std::slice;
use std::mem::transmute;

#[repr(transparent)]
pub struct StrUtf16 {
	inner: [u16]
}

impl StrUtf16 {
	#[inline]
	pub const fn from_utf16(code_units: &[u16]) -> Option<&Self> {
		if _internal::validate_utf16(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf16_unchecked(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf16_mut(code_units: &mut [u16]) -> Option<&mut Self> {
		if _internal::validate_utf16(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf16_unchecked_mut(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf16_unchecked(utf16: &[u16]) -> &Self {
		// SAFETY: [u16] and Self have same layout
		transmute(utf16)
	}

	#[inline]
	pub unsafe fn from_utf16_unchecked_mut(utf16: &mut [u16]) -> &mut Self {
		// SAFETY: [u16] and Self have same layout
		transmute(utf16)
	}

	#[inline]
	pub const fn to_utf16_code_units(&self) -> &[u16] {
		// SAFETY: [u16] and Self have same layout
		unsafe { transmute(self) }
	}

	#[inline]
	pub fn to_utf16le_bytes(&self) -> Cow<'_, [u8]> {
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

	#[inline]
	pub fn to_utf16be_bytes(&self) -> Cow<'_, [u8]> {
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

	#[inline]
	pub const fn len_code_units(&self) -> usize {
		self.to_utf16_code_units().len()
	}

	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.len_code_units() == 0
	}

	#[inline]
	pub const fn is_char_boundary(&self, index: usize) -> bool {
		if index <= self.len_code_units() {
			unsafe { _internal::is_char_boundary_utf16_unchecked(self.to_utf16_code_units(), index) }
		} else {
			false
		}
	}
}
