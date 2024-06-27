use std::mem::{ size_of, align_of };

/// Common base for all numbers
pub trait Base: Sized + Clone + private::Sealed {
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

mod private {
	pub trait Sealed {}
}

macro_rules! impl_num_trait_base {
	{
		int: unsigned $num:ident
	} => {
		impl_num_trait_base! {
			base: $num;
			zero: 0;
			one: 1;
			from_bool: b => b as _;
		}
	};

	{
		int: signed $num:ident
	} => {
		impl_num_trait_base! {
			base: $num;
			zero: 0;
			one: 1;
			from_bool: b => b as _;
		}
	};

	{
		float: $num:ident
	} => {
		impl_num_trait_base! {
			base: $num;
			zero: 0.0;
			one: 1.0;
			from_bool: b => if b { 1.0 } else { 0.0 };
		}
	};

	{
		base: $num:ident;
		zero: $zero:literal;
		one: $one:literal;
		from_bool: $b:ident => $from_bool:expr;
	} => {
		impl private::Sealed for $num {}
		impl Base for $num {
			const MIN: $num = $num::MIN;
			const MAX: $num = $num::MAX;
			const ZERO: $num = $zero;
			const ONE: $num = $one;
			#[allow(clippy::manual_bits)] // shut
			const BITS: usize = size_of::<$num>() * 8;
			const BYTES: usize = size_of::<$num>();
			const ALIGN: usize = align_of::<$num>();

			fn from_bool($b: bool) -> $num { $from_bool }
		}
	};
}

impl_num_trait_base! {
	int: unsigned u8
}

impl_num_trait_base! {
	int: unsigned u16
}

impl_num_trait_base! {
	int: unsigned u32
}

impl_num_trait_base! {
	int: unsigned u64
}

impl_num_trait_base! {
	int: unsigned u128
}

impl_num_trait_base! {
	int: unsigned usize
}

impl_num_trait_base! {
	int: signed i8
}

impl_num_trait_base! {
	int: signed i16
}

impl_num_trait_base! {
	int: signed i32
}

impl_num_trait_base! {
	int: signed i64
}

impl_num_trait_base! {
	int: signed i128
}

impl_num_trait_base! {
	int: signed isize
}

impl_num_trait_base! {
	float: f32
}

impl_num_trait_base! {
	float: f64
}
