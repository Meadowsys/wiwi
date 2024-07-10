#[repr(transparent)]
pub struct Str {
	inner: [u32]
}

pub struct String {
	inner: Vec<u32>
}

impl Str {
	#[inline]
	pub unsafe fn from_utf32_unchecked(utf32: &[u32]) -> &Str {
		&*(utf32 as *const [u32] as *const Str)
	}

	#[inline]
	pub unsafe fn from_utf32_unchecked_mut(utf32: &mut [u32]) -> &mut Str {
		&mut *(utf32 as *mut [u32] as *mut Str)
	}
}

impl String {
	#[inline]
	pub unsafe fn from_utf32_unchecked(utf32: Vec<u32>) -> String {
		String { inner: utf32 }
	}
}
