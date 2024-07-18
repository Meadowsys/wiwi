use super::_internal;
use std::borrow::Cow;
use std::slice;
use std::mem::transmute;

#[repr(transparent)]
pub struct StrUtf16 {
	inner: [u16]
}

pub struct StringUtf16 {
	inner: Vec<u16>
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
		unsafe { transmute(utf16) }
	}

	#[inline]
	pub unsafe fn from_utf16_unchecked_mut(utf16: &mut [u16]) -> &mut Self {
		// SAFETY: [u16] and Self have same layout
		unsafe { transmute(utf16) }
	}

	#[inline]
	pub const fn to_utf16_code_units(&self) -> &[u16] {
		// SAFETY: [u16] and Self have same layout
		unsafe { transmute(self) }
	}

	#[inline]
	pub fn to_utf16le_bytes(&self) -> Cow<'_, [u8]> {
		// TODO: this can be unchecked mul
		let byte_len = self.len_code_units() * 2;

		// if we're on little endian platform, we can just provide byte view
		// into the same slice
		#[cfg(target_endian = "little")]
		let rv = unsafe {
			let ptr = self.to_utf16_code_units() as *const [u16] as *const u8;

			// SAFETY:
			// - u8 is half the size of u16, so len of resulting slice being
			//   twice len will refer to same region of memory
			// - u16 has more alignment than u8, so this alignment is fine
			// - we're only borrowing / providing a different view on the same memory
			//   owned by someone else, so we don't have to worry about eg. deallocation
			//   must be using the same layout
			Cow::Borrowed(slice::from_raw_parts(ptr, byte_len))
		};

		// ... but if we're not on little endian...
		// we gotta make a copy and flip the code units
		#[cfg(not(target_endian = "little"))]
		let rv = unsafe {
			// create vec with enough capacity (u8 is half size, so needs twice
			// amount to get same byte size of capacity)
			let mut le_bytes = Vec::<u8>::with_capacity(byte_len);

			// SAFETY: this resulting ptr is safe to write to:
			// - [u8; 2] has same alignment as u8, so the ptr is aligned
			// - [u8; 2] is has the same size as [u16]
			// and, we cast to [u8; 2] because u16::to_le_bytes returns [u8; 2],
			// so we can do a simple .add call on the ptr to get to the right slot
			let le_bytes_ptr = le_bytes.as_mut_ptr() as *mut [u8; 2];

			// SAFETY: we're iter() on the code units slice, which is what we
			// based the length on initially, so the vec pointer will be
			// guaranteed to have enough space to write to
			self.to_utf16_code_units()
				.iter()
				.copied()
				.enumerate()
				// to_le_bytes here flips the bytes before returning the array to write
				.for_each(|(i, cu)| le_bytes_ptr.add(i).write(cu.to_le_bytes()));

			Cow::Owned(le_bytes)
		};

		rv
	}

	#[inline]
	pub fn to_utf16be_bytes(&self) -> Cow<'_, [u8]> {
		// TODO: this can be unchecked mul
		let byte_len = self.len_code_units() * 2;

		// if we're on big endian platform, we can just provide byte view
		// into the same slice
		#[cfg(target_endian = "big")]
		let rv = unsafe {
			let ptr = self.to_utf16_code_units() as *const [u16] as *const u8;

			// SAFETY:
			// - u8 is half the size of u16, so len of resulting slice being
			//   twice len will refer to same region of memory
			// - u16 has more alignment than u8, so this alignment is fine
			// - we're only borrowing / providing a different view on the same memory
			//   owned by someone else, so we don't have to worry about eg. deallocation
			//   must be using the same layout
			Cow::Borrowed(slice::from_raw_parts(ptr, byte_len))
		};

		// ... but if we're not on big endian...
		// we gotta make a copy and flip the code units
		#[cfg(not(target_endian = "big"))]
		let rv = unsafe {
			// create vec with enough capacity (u8 is half size, so needs twice
			// amount to get same byte size of capacity)
			let mut be_bytes = Vec::<u8>::with_capacity(byte_len);

			// SAFETY: this resulting ptr is safe to write to:
			// - [u8; 2] has same alignment as u8, so the ptr is aligned
			// - [u8; 2] is has the same size as u16
			// and, we cast to [u8; 2] because u16::to_le_bytes returns [u8; 2],
			// so we can do a simple .add call on the ptr to get to the right slot
			let be_bytes_ptr = be_bytes.as_mut_ptr() as *mut [u8; 2];

			// SAFETY: we're iter() on the code units slice, which is what we
			// based the length on initially, so the vec pointer will be
			// guaranteed to have enough space to write to
			self.to_utf16_code_units()
				.iter()
				.copied()
				.enumerate()
				// to_be_bytes here flips the bytes before returning the array to write
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
		// SAFETY: `to_utf16_code_units` returns valid UTF-16 code units
		// (well, `self` must be valid UTF-16)
		unsafe { _internal::is_char_boundary_utf16_unchecked(self.to_utf16_code_units(), index) }
	}
}

impl StringUtf16 {
	#[inline]
	pub const fn new() -> Self {
		Self { inner: Vec::new() }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		Self { inner: Vec::with_capacity(capacity) }
	}
}

impl Default for StringUtf16 {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}
