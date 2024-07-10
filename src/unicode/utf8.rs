#[repr(transparent)]
pub struct Str {
	inner: [u8]
}

pub struct String {
	inner: Vec<u8>
}

impl Str {
	#[inline]
	pub unsafe fn from_utf8_unchecked(utf8: &[u8]) -> &Str {
		&*(utf8 as *const [u8] as *const Str)
	}

	#[inline]
	pub unsafe fn from_utf8_unchecked_mut(utf8: &mut [u8]) -> &mut Str {
		&mut *(utf8 as *mut [u8] as *mut Str)
	}
}

impl String {
	#[inline]
	pub unsafe fn from_utf8_unchecked(utf8: Vec<u8>) -> String {
		String { inner: utf8 }
	}
}
