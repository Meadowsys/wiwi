use std::mem::{ size_of, align_of };

/// Common base for all numbers
pub trait Base: Sized {
	/// Min (finite) value of this number type
	const MIN: Self;
	/// Max (finite) value of this number type
	const MAX: Self;
	/// `0`
	const ZERO: Self;
	/// `1`
	const ONE: Self;
	/// Size of this number in bits
	const BITS: usize;
	/// Size of this number type in bytes
	const BYTES: usize;
	/// Align of this number type in bytes
	const ALIGN: usize;
}

macro_rules! impl_num_trait_base {
	{ $($num:ident)* } => {
		$(
			impl Base for $num {
				const MIN: $num = $num::MIN;
				const MAX: $num = $num::MAX;
				const ZERO: $num = 0 as _;
				const ONE: $num = 1 as _;
				// shut up I can't
				// floats don't have ::BITS
				#[allow(clippy::manual_bits)]
				const BITS: usize = size_of::<$num>() * 8;
				const BYTES: usize = size_of::<$num>();
				const ALIGN: usize = align_of::<$num>();
			}
		)*
	}
}

impl_num_trait_base! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}
