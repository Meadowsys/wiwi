use crate::bitstream::Encoder;
use super::{ IntoChainer, VecChain };

#[must_use = include_str!("./must-use-msg.txt")]
#[repr(transparent)]
pub struct BitstreamEncoderChainer {
	inner: Encoder
}

/// Constructor functions
impl BitstreamEncoderChainer {
	pub fn new() -> Self {
		Encoder::new().into()
	}

	pub fn with_output_capacity(capacity: usize) -> Self {
		Encoder::with_output_capacity(capacity).into()
	}
}

/// Conversion functions
impl BitstreamEncoderChainer {
	pub fn into_bytes(self) -> Vec<u8> {
		self.inner.into_bytes()
	}

	pub fn into_byte_chainer(self) -> VecChain<u8> {
		self.into_bytes().into()
	}
}

/// Chaining functions
impl BitstreamEncoderChainer {
	pub unsafe fn write_bits_u8_unchecked(mut self, num_bits: usize, bits: u8) -> Self {
		self.inner.write_bits_u8_unchecked(num_bits, bits);
		self
	}

	pub unsafe fn write_bits_u16_unchecked(mut self, num_bits: usize, bits: u16) -> Self {
		self.inner.write_bits_u16_unchecked(num_bits, bits);
		self
	}

	pub unsafe fn write_bits_u32_unchecked(mut self, num_bits: usize, bits: u32) -> Self {
		self.inner.write_bits_u32_unchecked(num_bits, bits);
		self
	}

	pub unsafe fn write_bits_u64_unchecked(mut self, num_bits: usize, bits: u64) -> Self {
		self.inner.write_bits_u64_unchecked(num_bits, bits);
		self
	}

	pub unsafe fn write_bits_u128_unchecked(mut self, num_bits: usize, bits: u128) -> Self {
		self.inner.write_bits_u128_unchecked(num_bits, bits);
		self
	}
}

impl IntoChainer for Encoder {
	type Chain = BitstreamEncoderChainer;
	fn into_chainer(self) -> BitstreamEncoderChainer {
		self.into()
	}
}

impl From<Encoder> for BitstreamEncoderChainer {
	#[inline]
	fn from(inner: Encoder) -> Self {
		Self { inner }
	}
}
