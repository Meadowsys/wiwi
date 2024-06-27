use std::mem::{ size_of, align_of };

/// Common base for all numbers
pub trait Base: Sized + private::Sealed {
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

	fn from_bool(b: bool) -> Self;
}

macro_rules! impl_num_trait_base {
	{ @base $zero:literal $one:literal $num:ident } => {
		const MIN: $num = $num::MIN;
		const MAX: $num = $num::MAX;
		const ZERO: $num = $zero;
		const ONE: $num = $one;
		// shut up I can't
		// floats don't have ::BITS
		#[allow(clippy::manual_bits)]
		const BITS: usize = size_of::<$num>() * 8;
		const BYTES: usize = size_of::<$num>();
		const ALIGN: usize = align_of::<$num>();
	};

	{ @floats $($num:ident)* } => {
		$(
			impl private::Sealed for $num {}
			impl Base for $num {
				impl_num_trait_base! { @base 0.0 1.0 $num }

				#[inline(always)]
				fn from_bool(b: bool) -> $num {
					if b { Self::ONE } else { Self::ZERO }
				}
			}
		)*
	};

	{ $($num:ident)* } => {
		$(
			impl private::Sealed for $num {}
			impl Base for $num {
				impl_num_trait_base! { @base 0 1 $num }

				#[inline(always)]
				fn from_bool(b: bool) -> $num { b as _ }
			}
		)*
	};
}

impl_num_trait_base! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

impl_num_trait_base! {
	@floats f32 f64
}

mod private {
	pub trait Sealed {}
}
