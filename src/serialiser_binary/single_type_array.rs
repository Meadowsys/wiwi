use super::internal_prelude::*;
use super::USizeSerialiser;
use super::number::{
	get_byte_count_signed_le,
	get_byte_count_unsigned_le,
	get_marker_for_signed,
	get_marker_for_unsigned
};
use std::{ hint, slice };

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

macro_rules! impl_serialise {
	{ $($int:ident)* } => {
		$(
			impl Serialise for SingleTypeArray<&[$int]> {
				type Serialiser<'h> = SingleTypeArraySerialiserInt<'h, $int> where Self: 'h;

				fn build_serialiser(&self) -> SingleTypeArraySerialiserInt<'_, $int> {
					SingleTypeArraySerialiserInt::<$int>::new(self.0)
				}
			}

			impl Serialise for SingleTypeArray<Vec<$int>> {
				type Serialiser<'h> = SingleTypeArraySerialiserInt<'h, $int> where Self: 'h;

				fn build_serialiser(&self) -> SingleTypeArraySerialiserInt<'_, $int> {
					SingleTypeArraySerialiserInt::<$int>::new(&self.0)
				}
			}

			impl Serialise for SingleTypeArray<Box<[$int]>> {
				type Serialiser<'h> = SingleTypeArraySerialiserInt<'h, $int> where Self: 'h;

				fn build_serialiser(&self) -> SingleTypeArraySerialiserInt<'_, $int> {
					SingleTypeArraySerialiserInt::<$int>::new(&self.0)
				}
			}

			impl Serialise for SingleTypeArrayExpensive<&[$int]> {
				type Serialiser<'h> = SingleTypeArraySerialiserInt<'h, $int> where Self: 'h;

				fn build_serialiser(&self) -> SingleTypeArraySerialiserInt<'_, $int> {
					SingleTypeArraySerialiserInt::<$int>::new(self.0)
				}
			}

			impl Serialise for SingleTypeArrayExpensive<Vec<$int>> {
				type Serialiser<'h> = SingleTypeArraySerialiserInt<'h, $int> where Self: 'h;

				fn build_serialiser(&self) -> SingleTypeArraySerialiserInt<'_, $int> {
					SingleTypeArraySerialiserInt::<$int>::new(&self.0)
				}
			}

			impl Serialise for SingleTypeArrayExpensive<Box<[$int]>> {
				type Serialiser<'h> = SingleTypeArraySerialiserInt<'h, $int> where Self: 'h;

				fn build_serialiser(&self) -> SingleTypeArraySerialiserInt<'_, $int> {
					SingleTypeArraySerialiserInt::<$int>::new(&self.0)
				}
			}
		)*
	}
}

impl_serialise! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

pub enum SingleTypeArraySerialiserInt<'h, T>
where
	[T]: Serialise
{
	SingleTypeArray {
		slice: &'h [T],
		len_ser: Option<USizeSerialiser>,
		reduced_count: u8
	},
	PlainArray {
		plain_array_ser: <&'h [T] as Serialise>::Serialiser<'h>
	}
}

impl<'h, T> SingleTypeArraySerialiserInt<'h, T>
where
	[T]: Serialise
{
	fn _new(slice: &'h [T], reduced_count: u8) -> Self {
		let len_ser = if slice.len() > u8::MAX.into_usize() {
			Some(USizeSerialiser::new(slice.len()))
		} else {
			None
		};

		Self::SingleTypeArray { slice, len_ser, reduced_count }
	}
}

impl<'h> SingleTypeArraySerialiserInt<'h, u8> {
	fn new(slice: &'h [u8]) -> Self {
		Self::_new(slice, 1)
	}

	fn new_expensive(slice: &'h [u8]) -> Self {
		Self::_new(slice, 1)
	}
}

impl<'h> Serialiser<'h> for SingleTypeArraySerialiserInt<'h, u8> {
	unsafe fn needed_capacity(&self) -> usize {
		match self {
			Self::SingleTypeArray { slice, len_ser, reduced_count: _ } => {
				if let Some(len_ser) = len_ser {
					// array marker (1) + array len + type marker (1) + stuff (1 each)
					1 + len_ser.needed_capacity() + 1 + slice.len()
				} else {
					// array marker (1) + array len (1) + type marker (1) + stuff (1 each)
					1 + 1 + 1 + slice.len()
				}
			}
			Self::PlainArray { plain_array_ser } => {
				plain_array_ser.needed_capacity()
			}
		}
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		match self {
			Self::SingleTypeArray { slice, len_ser, reduced_count: _ } => {
				if let Some(len_ser) = len_ser {
					buf.write_byte(MARKER_SINGLE_TYPE_ARRAY_XL);
					len_ser.serialise(buf);
				} else {
					buf.write_byte(MARKER_SINGLE_TYPE_ARRAY_8);
					buf.write_byte(slice.len().into_u8_lossy());
				}

				buf.write_byte(MARKER_U8);
				buf.write_bytes(slice);
			}
			Self::PlainArray { plain_array_ser } => {
				plain_array_ser.serialise(buf)
			}
		}
	}
}

impl<'h> SingleTypeArraySerialiserInt<'h, i8> {
	fn new(slice: &'h [i8]) -> Self {
		Self::_new(slice, 1)
	}

	fn new_expensive(slice: &'h [i8]) -> Self {
		Self::_new(slice, 1)
	}
}

impl<'h> Serialiser<'h> for SingleTypeArraySerialiserInt<'h, i8> {
	unsafe fn needed_capacity(&self) -> usize {
		match self {
			Self::SingleTypeArray { slice, len_ser, reduced_count: _ } => {
				if let Some(len_ser) = len_ser {
					// array marker (1) len + type marker (1) + stuff (1 each)
					1 + len_ser.needed_capacity() + 1 + slice.len()
				} else {
					// array marker (1) + array len (1) + type marker (1) + stuff (1 each)
					1 + 1 + 1 + slice.len()
				}
			}
			Self::PlainArray { plain_array_ser } => {
				plain_array_ser.needed_capacity()
			}
		}
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		match self {
			Self::SingleTypeArray { slice, len_ser, reduced_count: _ } => {
				if let Some(len_ser) = len_ser {
					buf.write_byte(MARKER_SINGLE_TYPE_ARRAY_XL);
					len_ser.serialise(buf);
				} else {
					buf.write_byte(MARKER_SINGLE_TYPE_ARRAY_8);
					buf.write_byte(slice.len().into_u8_lossy());
				}

				buf.write_byte(MARKER_I8);
				buf.write_bytes(&*(*slice as *const [i8] as *const [u8]));
			}
			Self::PlainArray { plain_array_ser } => {
				plain_array_ser.serialise(buf)
			}
		}
	}
}

macro_rules! impl_sta_seriraliser {
	{ unsigned $($int:ident)* } => {
		impl_sta_seriraliser! {
			@impl
			$($int get_byte_count_unsigned_le get_marker_for_unsigned)*
		}
	};

	{ signed $($int:ident)* } => {
		impl_sta_seriraliser! {
			@impl
			$($int get_byte_count_signed_le get_marker_for_signed)*
		}
	};

	{ @impl $($int:ident $count_fn:ident $marker_fn:ident)* } => {
		$(
			impl<'h> SingleTypeArraySerialiserInt<'h, $int> {
				fn new(slice: &'h [$int]) -> Self {
					Self::_new(slice, size_of::<$int>().into_u8_lossy())
				}

				fn new_expensive(slice: &'h [$int]) -> Self {
					let reduced_count = slice.iter()
						.copied()
						.map(|val| unsafe { $count_fn(val.to_le_bytes()) })
						.max()
						.unwrap_or(1);

					let mut single_type_ser = Self::_new(slice, reduced_count);
					let plain_array_ser = slice.build_serialiser();

					let plain_array_cap = unsafe { plain_array_ser.needed_capacity() };
					let single_type_cap = unsafe { single_type_ser.needed_capacity() };

					if plain_array_cap < single_type_cap {
						single_type_ser = Self::PlainArray { plain_array_ser };
					}

					single_type_ser
				}
			}

			impl<'h> Serialiser<'h> for SingleTypeArraySerialiserInt<'h, $int> {
				unsafe fn needed_capacity(&self) -> usize {
					match self {
						Self::SingleTypeArray { slice, len_ser, reduced_count } => {
							let slice_len = (slice.len() * reduced_count.into_usize());

							if let Some(len_ser) = len_ser {
								// array marker (1) + array len + type marker (1) + stuff
								1 + len_ser.needed_capacity() + 1 + slice_len
							} else {
								// array marker (1) + array len (1) + type marker (1) + stuff
								1 + 1 + 1 + slice_len
							}
						}
						Self::PlainArray { plain_array_ser } => {
							plain_array_ser.needed_capacity()
						}
					}
				}

				unsafe fn serialise<O: Output>(&self, buf: &mut O) {
					match self {
						Self::SingleTypeArray { slice, len_ser, reduced_count } => {
							if reduced_count.into_usize() > size_of::<$int>() { hint::unreachable_unchecked() }

							if let Some(len_ser) = len_ser {
								buf.write_byte(MARKER_SINGLE_TYPE_ARRAY_XL);
								len_ser.serialise(buf);
							} else {
								buf.write_byte(MARKER_SINGLE_TYPE_ARRAY_8);
								buf.write_byte(slice.len().into_u8_lossy());
							}

							buf.write_byte($marker_fn(*reduced_count));

							#[cfg(target_endian = "little")] {
								// on little endian arch, if we don't use a reduced size
								// type, we can just straight bitwise copy in
								if reduced_count.into_usize() == size_of::<$int>() {
									let slice = slice::from_raw_parts(
										slice.as_ptr().cast::<u8>(),
										slice.len() * size_of::<$int>()
									);
									buf.write_bytes(slice);
									return;
								}
							}

							slice.iter()
								.copied()
								.for_each(|val| {
									buf.write_bytes(
										val.to_le_bytes()
											.get_unchecked(..reduced_count.into_usize())
									);
								});
						}
						Self::PlainArray { plain_array_ser } => {
							plain_array_ser.serialise(buf)
						}
					}
				}
			}
		)*
	};
}

impl_sta_seriraliser! {
	unsigned
	u16 u32 u64 u128 usize
}

impl_sta_seriraliser! {
	signed
	i16 i32 i64 i128 isize
}
