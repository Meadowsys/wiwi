use super::internal_prelude::*;
use super::BinarySerialiser;

/// A wrapper type for items that can be serialised into a single type array.
///
/// A single type array is an array with elements of only a single type, with
/// the type marker specified only once at the beginning, and contiguous elements
/// for rest of it. The longer the array, the higher the potential space savings.
///
/// Some types can be "compressed" (ie. a u32 with a small value can be stored
/// in a smaller serialised type). Depending on the data, using a single type array
/// could mean more space usage, since for these "compressable" types, the single
/// type array must pick the "lowest common denominator", that is, the smallest
/// type that is large enough to fit all numbers into it.
///
/// For example, a u32 array of `[u32::MAX, 0, 0, 0]` can serialise into 11 bytes
/// as a regular array, but serialising as a single type array will result in
/// 23 bytes(!) of serialised output.
///
/// This type is the "cheapest" implementation of a single type array, as it simply
/// picks a type that can fit all possible values. If you would like more aggressive
/// picking for more serialise cost, you can use [`SingleTypeArrayExpensive`].
#[repr(transparent)]
pub struct SingleTypeArray<T>(T);

impl<T> SingleTypeArray<T> {
	pub fn new(val: T) -> Self
	where
		Self: Serialise
	{
		Self(val)
	}
}

impl Serialise for SingleTypeArray<&[u8]> {
	type Serialiser<'h> = BinarySerialiser<'h> where Self: 'h;

	fn build_serialiser(&self) -> BinarySerialiser<'_> {
		BinarySerialiser::new(self.0)
	}
}

impl Serialise for SingleTypeArray<Vec<u8>> {
	type Serialiser<'h> = BinarySerialiser<'h> where Self: 'h;

	fn build_serialiser(&self) -> BinarySerialiser<'_> {
		BinarySerialiser::new(&self.0)
	}
}

/// A wrapper type for items that can be serialised into a single type array.
///
/// More compute-expensive version of [`SingleTypeArray`], for more space savings.
/// See that struct for more info.
#[repr(transparent)]
pub struct SingleTypeArrayExpensive<T>(T);

impl<T> SingleTypeArrayExpensive<T> {
	pub fn new(val: T) -> Self
	where
		Self: Serialise
	{
		Self(val)
	}
}

impl Serialise for SingleTypeArrayExpensive<&[u8]> {
	type Serialiser<'h> = BinarySerialiser<'h> where Self: 'h;

	fn build_serialiser(&self) -> BinarySerialiser<'_> {
		BinarySerialiser::new(self.0)
	}
}

impl Serialise for SingleTypeArrayExpensive<Vec<u8>> {
	type Serialiser<'h> = BinarySerialiser<'h> where Self: 'h;

	fn build_serialiser(&self) -> BinarySerialiser<'_> {
		BinarySerialiser::new(&self.0)
	}
}
