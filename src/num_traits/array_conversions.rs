use super::Base;

/// Array conversions
///
/// To use this trait as a trait bound on something, also declare a `usize`
/// const generic parameter, and then use it to specify the bound on your
/// generic parameter. For example:
///
/// ```
/// # use wiwi::num_traits::ArrayConversions;
/// //                 ↓ your parameter    ↓↓ const generic ↓↓
/// fn number_function<T: ArrayConversions<BYTES>, const BYTES: usize>(num: T) {
///    let array: [u8; BYTES] = num.into_le_bytes();
/// }
/// ```
pub trait ArrayConversions<const BYTES: usize>: Base {
	/// Converts this number into a byte array in little endian order
	fn into_le_bytes(self) -> [u8; BYTES];
	/// Converts this number into a byte array in big endian order
	fn into_be_bytes(self) -> [u8; BYTES];
	/// Converts this number into a byte array in native endian order
	fn into_ne_bytes(self) -> [u8; BYTES];

	/// Converts an array of bytes in little endian order to a number
	fn from_le_bytes(bytes: [u8; BYTES]) -> Self;
	/// Converts an array of bytes in big endian order to a number
	fn from_be_bytes(bytes: [u8; BYTES]) -> Self;
	/// Converts an array of bytes in native endian order to a number
	fn from_ne_bytes(bytes: [u8; BYTES]) -> Self;
}

macro_rules! impl_num_trait_array_conversions {
	{ $($num:ident)* } => {
		$(
			impl ArrayConversions<{ $num::BYTES }> for $num {
				fn into_le_bytes(self) -> [u8; $num::BYTES] { $num::to_le_bytes(self) }
				fn into_be_bytes(self) -> [u8; $num::BYTES] { $num::to_be_bytes(self) }
				fn into_ne_bytes(self) -> [u8; $num::BYTES] { $num::to_ne_bytes(self) }

				fn from_le_bytes(bytes: [u8; $num::BYTES]) -> $num { $num::from_le_bytes(bytes) }
				fn from_be_bytes(bytes: [u8; $num::BYTES]) -> $num { $num::from_be_bytes(bytes) }
				fn from_ne_bytes(bytes: [u8; $num::BYTES]) -> $num { $num::from_ne_bytes(bytes) }
			}
		)*
	}
}

impl_num_trait_array_conversions! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}
