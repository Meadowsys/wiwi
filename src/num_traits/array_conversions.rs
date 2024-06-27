use super::Base;

/// Array conversions
pub trait ArrayConversions<const BYTES: usize>: Sized + Base {
	fn into_le_bytes(self) -> [u8; BYTES];
	fn into_be_bytes(self) -> [u8; BYTES];
	fn into_ne_bytes(self) -> [u8; BYTES];

	fn from_le_bytes(bytes: [u8; BYTES]) -> Self;
	fn from_be_bytes(bytes: [u8; BYTES]) -> Self;
	fn from_ne_bytes(bytes: [u8; BYTES]) -> Self;
}
