use std::mem::transmute;

#[repr(transparent)]
pub struct Str {
	inner: [u8]
}

pub struct String {
	inner: Vec<u8>
}

impl Str {
	#[inline]
	pub const unsafe fn from_utf8_unchecked(utf8: &[u8]) -> &Str {
		transmute(utf8)
	}

	#[inline]
	pub unsafe fn from_utf8_unchecked_mut(utf8: &mut [u8]) -> &mut Str {
		transmute(utf8)
	}
}

impl String {
	#[inline]
	pub unsafe fn from_utf8_unchecked(utf8: Vec<u8>) -> String {
		String { inner: utf8 }
	}
}
