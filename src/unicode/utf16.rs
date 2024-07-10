#[repr(transparent)]
pub struct Str {
	inner: [u16]
}

pub struct String {
	inner: Vec<u16>
}

impl Str {
	#[inline]
	pub unsafe fn from_utf16_unchecked(utf16: &[u16]) -> &Str {
		&*(utf16 as *const [u16] as *const Str)
	}

	#[inline]
	pub unsafe fn from_utf16_unchecked_mut(utf16: &mut [u16]) -> &mut Str {
		&mut *(utf16 as *mut [u16] as *mut Str)
	}
}

impl String {
	#[inline]
	pub unsafe fn from_utf16_unchecked(utf16: Vec<u16>) -> String {
		String { inner: utf16 }
	}
}
