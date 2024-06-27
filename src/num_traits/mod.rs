#![deny(unconditional_recursion)]

use std::cmp::{ Eq, Ord, PartialEq, PartialOrd };
use std::fmt::{ Debug, Display };
use std::hash::Hash;
use std::iter::{ Sum, Product };
use std::ops;
/*
pub trait Int<const BYTES: usize>: Sized
where
	Self: Copy + Debug + Display + Default + Hash,
	Self: PartialEq<Self> + Eq + PartialOrd<Self> + Ord,
	Self: ops::Add<Self, Output = Self> + ops::AddAssign<Self>,
	Self: ops::Sub<Self, Output = Self> + ops::SubAssign<Self>,
	Self: ops::Mul<Self, Output = Self> + ops::MulAssign<Self>,
	Self: ops::Div<Self, Output = Self> + ops::DivAssign<Self>,
	Self: ops::Rem<Self, Output = Self> + ops::RemAssign<Self>,
	Self: ops::Shl<Self, Output = Self> + ops::ShlAssign<Self>,
	Self: ops::Shr<Self, Output = Self> + ops::ShrAssign<Self>,
	Self: ops::Not<Output = Self>,
	Self: ops::BitAnd<Self, Output = Self> + ops::BitAndAssign<Self>,
	Self: ops::BitOr<Self, Output = Self> + ops::BitOrAssign<Self>,
	Self: ops::BitXor<Self, Output = Self> + ops::BitXorAssign<Self>,
	Self: Sum<Self> + Product<Self>
{
	/// The smallest value that can be represented by this int type
	const MIN: Self;
	/// The largest value that can be represented by this int type
	const MAX: Self;
	/// The size of this integer type in bits
	const BITS: Self;
	/// The size of this integer type in bytes
	// TODO: this / generic param ehh weird funny
	const BYTES: Self;
	/// Zero
	const ZERO: Self;
	/// One
	const ONE: Self;

	/// Cast a boolean into `Self`
	///
	/// This returns `0` if `b` is `false`, and `1` if `b` is `true`
	fn from_bool(b: bool) -> Self;

	/// Count the amount of ones in the binary representation of `self`
	fn count_ones(self) -> Self;
	/// Count the amount of zeros in the binary representation of `self`
	fn count_zeros(self) -> Self;

	/// Count the amount of leading zeros in the binary representation of `self`
	fn leading_zeros(self) -> Self;
	/// Count the amount of trailing zeros in the binary representation of `self`
	fn trailing_zeros(self) -> Self;

	/// Count the amount of leading ones in the binary representation of `self`
	fn leading_ones(self) -> Self;
	/// Count the amount of trailing ones in the binary representation of `self`
	fn trailing_ones(self) -> Self;

	/// Shifts the bits left by a specified amount, wrapping the truncated bits
	/// around to the right
	fn rotate_left(self, n: Self) -> Self;
	/// Shifts the bits right by a specified amount, wrapping the truncated bits
	/// around to the left
	fn rotate_right(self, n: Self) -> Self;

	/// Converts an integer from big endian to native endian
	///
	/// On big endian, this does nothing, and on little endian, this swaps the bytes
	fn from_be(x: Self) -> Self;
	/// Converts an integer from little endian to native endian
	///
	/// On little endian, this does nothing, and on big endian, this swaps the bytes
	fn from_le(x: Self) -> Self;
	/// Converts `self` from native endian to big endian
	///
	/// On big endian, this does nothing, and on little endian, this swaps the bytes
	fn to_be(self) -> Self;
	/// Converts `self` from native endian to little endian
	///
	/// On little endian, this does nothing, and on big endian, this swaps the bytes
	fn to_le(self) -> Self;

	/// Takes a big endian byte array, and creates a native endian integer from it
	fn from_be_bytes(bytes: [u8; BYTES]) -> Self;
	/// Takes a little endian byte array, and creates a native endian integer from it
	fn from_le_bytes(bytes: [u8; BYTES]) -> Self;
	/// Takes a native endian byte array, and creates a native endian integer from it
	fn from_ne_bytes(bytes: [u8; BYTES]) -> Self;

	/// Converts `self` to its big endian byte array representation
	fn to_be_bytes(self) -> [u8; BYTES];
	/// Converts `self` to its little endian byte array representation
	fn to_le_bytes(self) -> [u8; BYTES];
	/// Converts `self` to its native endian byte array representation
	fn to_ne_bytes(self) -> [u8; BYTES];

	// TODO: below not in bigint

	fn checked_add(self, rhs: Self) -> Option<Self>;
	fn checked_sub(self, rhs: Self) -> Option<Self>;
	fn checked_mul(self, rhs: Self) -> Option<Self>;
	fn checked_div(self, rhs: Self) -> Option<Self>;
	fn checked_rem(self, rhs: Self) -> Option<Self>;
	fn checked_shl(self, rhs: Self) -> Option<Self>;
	fn checked_shr(self, rhs: Self) -> Option<Self>;
	fn checked_pow(self, rhs: Self) -> Option<Self>;

	// fn strict_add(self, rhs: Self) -> Self; // TODO nightly
	// fn strict_sub(self, rhs: Self) -> Self; // TODO nightly
	// fn strict_mul(self, rhs: Self) -> Self; // TODO nightly
	// fn strict_div(self, rhs: Self) -> Self; // TODO nightly
	// fn strict_rem(self, rhs: Self) -> Self; // TODO nightly
	// fn strict_shl(self, rhs: Self) -> Self; // TODO nightly
	// fn strict_shr(self, rhs: Self) -> Self; // TODO nightly
	// fn strict_pow(self, rhs: Self) -> Self; // TODO nightly

	unsafe fn unchecked_add(self, rhs: Self) -> Self;
	unsafe fn unchecked_sub(self, rhs: Self) -> Self;
	unsafe fn unchecked_mul(self, rhs: Self) -> Self;
	// unsafe fn unchecked_shl(self, rhs: Self) -> Self;
	// unsafe fn unchecked_shr(self, rhs: Self) -> Self;

	fn saturating_add(self, rhs: Self) -> Self;
	fn saturating_sub(self, rhs: Self) -> Self;
	fn saturating_mul(self, rhs: Self) -> Self;
	fn saturating_div(self, rhs: Self) -> Self;
	fn saturating_pow(self, rhs: Self) -> Self;

	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
	fn wrapping_mul(self, rhs: Self) -> Self;
	fn wrapping_div(self, rhs: Self) -> Self;
	fn wrapping_rem(self, rhs: Self) -> Self;
	fn wrapping_shl(self, rhs: Self) -> Self;
	fn wrapping_shr(self, rhs: Self) -> Self;
	fn wrapping_pow(self, rhs: Self) -> Self;

	fn overflowing_add(self, rhs: Self) -> (Self, bool);
	fn overflowing_sub(self, rhs: Self) -> (Self, bool);
	fn overflowing_mul(self, rhs: Self) -> (Self, bool);
	fn overflowing_div(self, rhs: Self) -> (Self, bool);
	fn overflowing_rem(self, rhs: Self) -> (Self, bool);
	fn overflowing_shl(self, rhs: Self) -> (Self, bool);
	fn overflowing_shr(self, rhs: Self) -> (Self, bool);
	fn overflowing_pow(self, rhs: Self) -> (Self, bool);
}

pub trait BigInt<const BYTES: usize>: Sized
where
	Self: Clone + Debug + Display + Default + Hash,
	Self: PartialEq<Self> + Eq + PartialOrd<Self> + Ord
{
	/// The smallest value that can be represented by this int type
	const MIN: Self;
	/// The largest value that can be represented by this int type
	const MAX: Self;
	/// The size of this integer type in bits
	const BITS: Self;
	/// The size of this integer type in bytes
	// TODO: this / generic param ehh weird funny
	const BYTES: Self;
	/// Zero
	const ZERO: Self;
	/// One
	const ONE: Self;

	/// Cast a boolean into `Self`
	fn from_bool(b: bool) -> Self;

	/// Count the amount of ones in the binary representation of `self`
	fn count_ones(&self) -> Self;
	/// Count the amount of zeros in the binary representation of `self`
	fn count_zeros(&self) -> Self;

	/// Count the amount of leading zeros in the binary representation of `self`
	fn leading_zeros(&self) -> Self;
	/// Count the amount of trailing zeros in the binary representation of `self`
	fn trailing_zeros(&self) -> Self;

	/// Count the amount of leading ones in the binary representation of `self`
	fn leading_ones(&self) -> Self;
	/// Count the amount of trailing ones in the binary representation of `self`
	fn trailing_ones(&self) -> Self;

	/// Shifts the bits left by a specified amount, wrapping the truncated bits
	/// around to the right, storing the result in `self`
	fn rotate_left(&mut self, n: &Self);
	/// Shifts the bits right by a specified amount, wrapping the truncated bits
	/// around to the left, storing the result in `self`
	fn rotate_right(&mut self, n: &Self);

	fn from_be_ref(x: &Self) -> Self;
	fn from_le_ref(x: &Self) -> Self;
	fn from_be(x: Self) -> Self;
	fn from_le(x: Self) -> Self;

	fn to_be(&self) -> Self;
	fn to_le(&self) -> Self;
	fn into_be(self) -> Self;
	fn into_le(self) -> Self;

	fn from_be_bytes_ref(bytes: &[u8; BYTES]) -> Self;
	fn from_le_bytes_ref(bytes: &[u8; BYTES]) -> Self;
	fn from_ne_bytes_ref(bytes: &[u8; BYTES]) -> Self;
	fn from_be_bytes(bytes: [u8; BYTES]) -> Self;
	fn from_le_bytes(bytes: [u8; BYTES]) -> Self;
	fn from_ne_bytes(bytes: [u8; BYTES]) -> Self;

	fn to_be_bytes(&self) -> [u8; BYTES];
	fn to_le_bytes(&self) -> [u8; BYTES];
	fn to_ne_bytes(&self) -> [u8; BYTES];
	fn into_be_bytes(self) -> [u8; BYTES];
	fn into_le_bytes(self) -> [u8; BYTES];
	fn into_ne_bytes(self) -> [u8; BYTES];

	fn checked_add(&self, rhs: &Self) -> Option<Self>;
	fn checked_sub(&self, rhs: &Self) -> Option<Self>;
	fn checked_mul(&self, rhs: &Self) -> Option<Self>;
	fn checked_div(&self, rhs: &Self) -> Option<Self>;
	fn checked_rem(&self, rhs: &Self) -> Option<Self>;
	fn checked_shl(&self, rhs: &Self) -> Option<Self>;
	fn checked_shr(&self, rhs: &Self) -> Option<Self>;
	fn checked_pow(&self, rhs: &Self) -> Option<Self>;

	// fn strict_add(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_sub(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_mul(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_div(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_rem(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_shl(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_shr(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_pow(&self, rhs: &Self) -> Self; // TODO nightly

	unsafe fn unchecked_add(&self, rhs: &Self) -> Self;
	unsafe fn unchecked_sub(&self, rhs: &Self) -> Self;
	unsafe fn unchecked_mul(&self, rhs: &Self) -> Self;
	// unsafe fn unchecked_shl(&self, rhs: &Self) -> Self;
	// unsafe fn unchecked_shr(&self, rhs: &Self) -> Self;

	fn saturating_add(&self, rhs: &Self) -> Self;
	fn saturating_sub(&self, rhs: &Self) -> Self;
	fn saturating_mul(&self, rhs: &Self) -> Self;
	fn saturating_div(&self, rhs: &Self) -> Self;
	fn saturating_pow(&self, rhs: &Self) -> Self;

	fn wrapping_add(&self, rhs: &Self) -> Self;
	fn wrapping_sub(&self, rhs: &Self) -> Self;
	fn wrapping_mul(&self, rhs: &Self) -> Self;
	fn wrapping_div(&self, rhs: &Self) -> Self;
	fn wrapping_rem(&self, rhs: &Self) -> Self;
	fn wrapping_shl(&self, rhs: &Self) -> Self;
	fn wrapping_shr(&self, rhs: &Self) -> Self;
	fn wrapping_pow(&self, rhs: &Self) -> Self;

	fn overflowing_add(&self, rhs: &Self) -> (Self, bool);
	fn overflowing_sub(&self, rhs: &Self) -> (Self, bool);
	fn overflowing_mul(&self, rhs: &Self) -> (Self, bool);
	fn overflowing_div(&self, rhs: &Self) -> (Self, bool);
	fn overflowing_rem(&self, rhs: &Self) -> (Self, bool);
	fn overflowing_shl(&self, rhs: &Self) -> (Self, bool);
	fn overflowing_shr(&self, rhs: &Self) -> (Self, bool);
	fn overflowing_pow(&self, rhs: &Self) -> (Self, bool);
}

impl<const BYTES: usize, T: Int<BYTES>> BigInt<BYTES> for T {
	const MIN: Self = <Self as Int<BYTES>>::MIN;
	const MAX: Self = <Self as Int<BYTES>>::MAX;
	const BITS: Self = <Self as Int<BYTES>>::BITS;
	const BYTES: Self = <Self as Int<BYTES>>::BYTES;
	const ZERO: Self = <Self as Int<BYTES>>::ZERO;
	const ONE: Self = <Self as Int<BYTES>>::ONE;

	fn from_bool(b: bool) -> Self { Int::from_bool(b) }

	fn count_ones(&self) -> Self { Int::count_ones(*self) }
	fn count_zeros(&self) -> Self { Int::count_zeros(*self) }
	fn leading_zeros(&self) -> Self { Int::leading_zeros(*self) }
	fn trailing_zeros(&self) -> Self { Int::trailing_zeros(*self) }
	fn leading_ones(&self) -> Self { Int::leading_ones(*self) }
	fn trailing_ones(&self) -> Self { Int::trailing_ones(*self) }

	fn rotate_left(&mut self, n: &Self) { *self = Int::rotate_left(*self, *n) }
	fn rotate_right(&mut self, n: &Self) { *self = Int::rotate_right(*self, *n) }

	fn from_be_ref(x: &Self) -> Self { Int::from_be(*x) }
	fn from_le_ref(x: &Self) -> Self { Int::from_le(*x) }
	fn from_be(x: Self) -> Self { Int::from_be(x) }
	fn from_le(x: Self) -> Self { Int::from_le(x) }

	fn to_be(&self) -> Self { Int::to_be(*self) }
	fn to_le(&self) -> Self { Int::to_le(*self) }
	fn into_be(self) -> Self { Int::to_be(self) }
	fn into_le(self) -> Self { Int::to_le(self) }

	fn from_be_bytes_ref(bytes: &[u8; BYTES]) -> Self { Int::from_be_bytes(*bytes) }
	fn from_le_bytes_ref(bytes: &[u8; BYTES]) -> Self { Int::from_le_bytes(*bytes) }
	fn from_ne_bytes_ref(bytes: &[u8; BYTES]) -> Self { Int::from_ne_bytes(*bytes) }
	fn from_be_bytes(bytes: [u8; BYTES]) -> Self { Int::from_be_bytes(bytes) }
	fn from_le_bytes(bytes: [u8; BYTES]) -> Self { Int::from_le_bytes(bytes) }
	fn from_ne_bytes(bytes: [u8; BYTES]) -> Self { Int::from_ne_bytes(bytes) }

	fn to_be_bytes(&self) -> [u8; BYTES] { Int::to_be_bytes(*self) }
	fn to_le_bytes(&self) -> [u8; BYTES] { Int::to_le_bytes(*self) }
	fn to_ne_bytes(&self) -> [u8; BYTES] { Int::to_ne_bytes(*self) }
	fn into_be_bytes(self) -> [u8; BYTES] { Int::to_be_bytes(self) }
	fn into_le_bytes(self) -> [u8; BYTES] { Int::to_le_bytes(self) }
	fn into_ne_bytes(self) -> [u8; BYTES] { Int::to_ne_bytes(self) }

	fn checked_add(&self, rhs: &Self) -> Option<Self> { Int::checked_add(*self, *rhs) }
	fn checked_sub(&self, rhs: &Self) -> Option<Self> { Int::checked_sub(*self, *rhs) }
	fn checked_mul(&self, rhs: &Self) -> Option<Self> { Int::checked_mul(*self, *rhs) }
	fn checked_div(&self, rhs: &Self) -> Option<Self> { Int::checked_div(*self, *rhs) }
	fn checked_rem(&self, rhs: &Self) -> Option<Self> { Int::checked_rem(*self, *rhs) }
	fn checked_shl(&self, rhs: &Self) -> Option<Self> { Int::checked_shl(*self, *rhs) }
	fn checked_shr(&self, rhs: &Self) -> Option<Self> { Int::checked_shr(*self, *rhs) }
	fn checked_pow(&self, rhs: &Self) -> Option<Self> { Int::checked_pow(*self, *rhs) }

	// fn strict_add(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_sub(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_mul(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_div(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_rem(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_shl(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_shr(&self, rhs: &Self) -> Self; // TODO nightly
	// fn strict_pow(&self, rhs: &Self) -> Self; // TODO nightly

	unsafe fn unchecked_add(&self, rhs: &Self) -> Self { Int::unchecked_add(*self, *rhs) }
	unsafe fn unchecked_sub(&self, rhs: &Self) -> Self { Int::unchecked_sub(*self, *rhs) }
	unsafe fn unchecked_mul(&self, rhs: &Self) -> Self { Int::unchecked_mul(*self, *rhs) }
	// unsafe fn unchecked_shl(&self, rhs: &Self) -> Self;
	// unsafe fn unchecked_shr(&self, rhs: &Self) -> Self;

	fn saturating_add(&self, rhs: &Self) -> Self { Int::saturating_add(*self, *rhs) }
	fn saturating_sub(&self, rhs: &Self) -> Self { Int::saturating_sub(*self, *rhs) }
	fn saturating_mul(&self, rhs: &Self) -> Self { Int::saturating_mul(*self, *rhs) }
	fn saturating_div(&self, rhs: &Self) -> Self { Int::saturating_div(*self, *rhs) }
	fn saturating_pow(&self, rhs: &Self) -> Self { Int::saturating_pow(*self, *rhs) }

	fn wrapping_add(&self, rhs: &Self) -> Self { Int::wrapping_add(*self, *rhs) }
	fn wrapping_sub(&self, rhs: &Self) -> Self { Int::wrapping_sub(*self, *rhs) }
	fn wrapping_mul(&self, rhs: &Self) -> Self { Int::wrapping_mul(*self, *rhs) }
	fn wrapping_div(&self, rhs: &Self) -> Self { Int::wrapping_div(*self, *rhs) }
	fn wrapping_rem(&self, rhs: &Self) -> Self { Int::wrapping_rem(*self, *rhs) }
	fn wrapping_shl(&self, rhs: &Self) -> Self { Int::wrapping_shl(*self, *rhs) }
	fn wrapping_shr(&self, rhs: &Self) -> Self { Int::wrapping_shr(*self, *rhs) }
	fn wrapping_pow(&self, rhs: &Self) -> Self { Int::wrapping_pow(*self, *rhs) }

	fn overflowing_add(&self, rhs: &Self) -> (Self, bool) { Int::overflowing_add(*self, *rhs) }
	fn overflowing_sub(&self, rhs: &Self) -> (Self, bool) { Int::overflowing_sub(*self, *rhs) }
	fn overflowing_mul(&self, rhs: &Self) -> (Self, bool) { Int::overflowing_mul(*self, *rhs) }
	fn overflowing_div(&self, rhs: &Self) -> (Self, bool) { Int::overflowing_div(*self, *rhs) }
	fn overflowing_rem(&self, rhs: &Self) -> (Self, bool) { Int::overflowing_rem(*self, *rhs) }
	fn overflowing_shl(&self, rhs: &Self) -> (Self, bool) { Int::overflowing_shl(*self, *rhs) }
	fn overflowing_shr(&self, rhs: &Self) -> (Self, bool) { Int::overflowing_shr(*self, *rhs) }
	fn overflowing_pow(&self, rhs: &Self) -> (Self, bool) { Int::overflowing_pow(*self, *rhs) }
}

// TODO: swap_bytes
// TODO: reverse_bits
// TODO: checked_add_signed
// TODO: strict_add_signed
// TODO: checked_div_euclid
// TODO: strict_div_euclid
// TODO: checked_rem_euclid
// TODO: strict_rem_euclid
// TODO: ilog
// TODO: ilog2
// TODO: ilog10
// TODO: checked_ilog
// TODO: checked_ilog2
// TODO: checked_ilog10
// TODO: checked_neg
// TODO: strict_neg
// TODO: saturating_add_signed
// TODO: wrapping_add_signed
// TODO: wrapping_div_euclid
// TODO: wrapping_rem_euclid
// TODO: wrapping_neg
// TODO: carrying_add
// TODO: overflowing_add_signed
// TODO: borrowing_sub
// TODO: abs_diff
// TODO: overflowing_div_euclid
// TODO: overflowing_rem_euclid
// TODO: overflowing_neg
// TODO: pow
// TODO: isqrt
// TODO: div_euclid
// TODO: rem_euclid
// TODO: div_floor
// TODO: div_ceil
// TODO: next_multiple_of
// TODO: checked_next_multiple_of
// TODO: is_power_of_two
// TODO: next_power_of_two
// TODO: checked_next_power_of_two
// TODO: wrapping_next_power_of_two
// TODO: midpoint


macro_rules! int_trait_impl {
	{ $($int:ident $signed:literal)* } => {
		$(
			impl Int<{ $int::BITS as usize / 8 }> for $int {
				const MIN: $int = $int::MIN;
				const MAX: $int = $int::MAX;
				const BITS: $int = $int::BITS as _;
				const BYTES: $int = {
					assert!($int::BITS % 8 == 0);
					$int::BITS as $int / 8
				};
				const ZERO: $int = 0;
				const ONE: $int = 1;

				fn from_bool(b: bool) -> Self { b as _ }

				int_trait_impl! {
					@fn(self) -> Self
					count_ones
					count_zeros
					leading_zeros
					trailing_zeros
					leading_ones
					trailing_ones
					to_be
					to_le
				}

				int_trait_impl! {
					@fn(self, Self) -> Self
					rotate_left
					rotate_right

					saturating_add
					saturating_sub
					saturating_mul
					saturating_div
					saturating_pow

					wrapping_add
					wrapping_sub
					wrapping_mul
					wrapping_div
					wrapping_rem
					wrapping_shl
					wrapping_shr
					wrapping_pow
				}

				int_trait_impl! {
					@fn(Self) -> Self
					from_be
					from_le
				}

				int_trait_impl! {
					@fn([u8; { $int::BITS as usize / 8 }]) -> Self
					from_be_bytes
					from_le_bytes
					from_ne_bytes
				}

				int_trait_impl! {
					@fn(self) -> [u8; { $int::BITS as usize / 8 }]
					to_be_bytes
					to_le_bytes
					to_ne_bytes
				}

				int_trait_impl! {
					@fn(self, Self) -> Option<Self>
					checked_add
					checked_sub
					checked_mul
					checked_div
					checked_rem
					checked_shl
					checked_shr
					checked_pow
				}

				int_trait_impl! {
					@unsafe fn(self, Self) -> Self
					unchecked_add
					unchecked_sub
					unchecked_mul
					// unchecked_shl
					// unchecked_shr
				}

				int_trait_impl! {
					@fn(self, Self) -> (Self, bool)
					overflowing_add
					overflowing_sub
					overflowing_mul
					overflowing_div
					overflowing_rem
					overflowing_shl
					overflowing_shr
					overflowing_pow
				}
			}
		)*
	};

	{ @fn(self) -> Self $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(self) -> Self {
				Self::$fn_name(self as _) as _
			}
		)*
	};

	{ @fn(self, Self) -> Self $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(self, x: Self) -> Self {
				Self::$fn_name(self, x as _) as _
			}
		)*
	};

	{ @fn(Self) -> Self $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(x: Self) -> Self {
				Self::$fn_name(x)
			}
		)*
	};

	{ @fn([u8; $bytes:expr]) -> Self $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(array: [u8; $bytes]) -> Self {
				Self::$fn_name(array)
			}
		)*
	};

	{ @fn(self) -> [u8; $bytes:expr] $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(self) -> [u8; $bytes] {
				Self::$fn_name(self)
			}
		)*
	};

	{ @fn(self, Self) -> Option<Self> $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(self, x: Self) -> Option<Self> {
				Self::$fn_name(self, x as _) as _
			}
		)*
	};

	{ @unsafe fn(self, Self) -> Self $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			unsafe fn $fn_name(self, x: Self) -> Self {
				Self::$fn_name(self, x as _) as _
			}
		)*
	};

	{ @fn(self, Self) -> (Self, bool) $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(self, x: Self) -> (Self, bool) {
				Self::$fn_name(self, x as _)
			}
		)*
	};
}

int_trait_impl! {
	u8 false
	u16 false
	u32 false
	u64 false
	u128 false

	i8 true
	i16 true
	i32 true
	i64 true
	i128 true
}
*/

pub trait UnsignedInt<const BYTES: usize>: Sized + sealed::UnsignedInt
where
	Self: Copy + Debug + Display + Default + Hash,
	Self: PartialEq<Self> + Eq + PartialOrd<Self> + Ord,
	Self: ops::Add<Self, Output = Self> + ops::AddAssign<Self>,
	Self: ops::Sub<Self, Output = Self> + ops::SubAssign<Self>,
	Self: ops::Mul<Self, Output = Self> + ops::MulAssign<Self>,
	Self: ops::Div<Self, Output = Self> + ops::DivAssign<Self>,
	Self: ops::Rem<Self, Output = Self> + ops::RemAssign<Self>,
	Self: ops::Shl<Self, Output = Self> + ops::ShlAssign<Self>,
	Self: ops::Shr<Self, Output = Self> + ops::ShrAssign<Self>,
	Self: ops::Not<Output = Self>,
	Self: ops::BitAnd<Self, Output = Self> + ops::BitAndAssign<Self>,
	Self: ops::BitOr<Self, Output = Self> + ops::BitOrAssign<Self>,
	Self: ops::BitXor<Self, Output = Self> + ops::BitXorAssign<Self>,
	Self: Sum<Self> + Product<Self>
{
	/// The smallest value that can be represented by this int type
	const MIN: Self;
	/// The largest value that can be represented by this int type
	const MAX: Self;
	/// The size of this integer type in bits
	const BITS: Self;
	/// The size of this integer type in bytes
	// TODO: this / generic param ehh weird funny
	const BYTES: Self;
	/// Zero
	const ZERO: Self;
	/// One
	const ONE: Self;

	/// Cast a boolean into `Self`
	///
	/// This returns `0` if `b` is `false`, and `1` if `b` is `true`
	fn from_bool(b: bool) -> Self;

	// reg checked unchecked strict overflowing saturating wrapping carrying
	// add sub mul div
	// rem shl shr pow
	// neg ilog ilog2 ilog10

	// fn add_regular(self, rhs: Self);
	// fn add_checked(self, rhs: Self);
	// unsafe fn add_unchecked(self, rhs: Self);
	// fn add_strict(self, rhs: Self);
	fn add_overflowing(self, rhs: Self) -> (Self, bool);
	// fn add_saturating(self, rhs: Self);
	// fn add_wrapping(self, rhs: Self);
	fn add_carrying(self, rhs: Self, carry: bool) -> (Self, bool);

	// fn sub_regular(self, rhs: Self);
	// fn sub_checked(self, rhs: Self);
	// unsafe fn sub_unchecked(self, rhs: Self);
	// fn sub_strict(self, rhs: Self);
	fn sub_overflowing(self, rhs: Self) -> (Self, bool);
	// fn sub_saturating(self, rhs: Self);
	// fn sub_wrapping(self, rhs: Self);
	fn sub_borrowing(self, rhs: Self, borrow: bool) -> (Self, bool);

	// fn mul_regular(self, rhs: Self);
	// fn mul_checked(self, rhs: Self);
	unsafe fn mul_unchecked(self, rhs: Self) -> Self;
	// fn mul_strict(self, rhs: Self);
	// fn mul_overflowing(self, rhs: Self);
	// fn mul_saturating(self, rhs: Self);
	// fn mul_wrapping(self, rhs: Self);
	// fn mul_carrying(self, rhs: Self);

	// fn div_regular(self, rhs: Self);
	// fn div_checked(self, rhs: Self);
	// unsafe fn div_unchecked(self, rhs: Self);
	// fn div_strict(self, rhs: Self);
	// fn div_overflowing(self, rhs: Self);
	// fn div_saturating(self, rhs: Self);
	// fn div_wrapping(self, rhs: Self);
	// fn div_carrying(self, rhs: Self);

	// fn rem_regular(self, rhs: Self);
	// fn rem_checked(self, rhs: Self);
	// unsafe fn rem_unchecked(self, rhs: Self);
	// fn rem_strict(self, rhs: Self);
	// fn rem_overflowing(self, rhs: Self);
	// fn rem_saturating(self, rhs: Self);
	// fn rem_wrapping(self, rhs: Self);
	// fn rem_carrying(self, rhs: Self);

	// fn shl_regular(self, rhs: Self);
	// fn shl_checked(self, rhs: Self);
	// unsafe fn shl_unchecked(self, rhs: Self);
	// fn shl_strict(self, rhs: Self);
	// fn shl_overflowing(self, rhs: Self);
	// fn shl_saturating(self, rhs: Self);
	// fn shl_wrapping(self, rhs: Self);
	// fn shl_carrying(self, rhs: Self);

	// fn shr_regular(self, rhs: Self);
	// fn shr_checked(self, rhs: Self);
	// unsafe fn shr_unchecked(self, rhs: Self);
	// fn shr_strict(self, rhs: Self);
	// fn shr_overflowing(self, rhs: Self);
	// fn shr_saturating(self, rhs: Self);
	// fn shr_wrapping(self, rhs: Self);
	// fn shr_carrying(self, rhs: Self);

	// fn pow_regular(self, rhs: Self);
	// fn pow_checked(self, rhs: Self);
	// unsafe fn pow_unchecked(self, rhs: Self);
	// fn pow_strict(self, rhs: Self);
	// fn pow_overflowing(self, rhs: Self);
	// fn pow_saturating(self, rhs: Self);
	// fn pow_wrapping(self, rhs: Self);
	// fn pow_carrying(self, rhs: Self);

	// fn neg_regular(self, rhs: Self);
	// fn neg_checked(self, rhs: Self);
	// unsafe fn neg_unchecked(self, rhs: Self);
	// fn neg_strict(self, rhs: Self);
	// fn neg_overflowing(self, rhs: Self);
	// fn neg_saturating(self, rhs: Self);
	// fn neg_wrapping(self, rhs: Self);
	// fn neg_carrying(self, rhs: Self);

	// fn ilog_regular(self, rhs: Self);
	// fn ilog_checked(self, rhs: Self);
	// unsafe fn ilog_unchecked(self, rhs: Self);
	// fn ilog_strict(self, rhs: Self);
	// fn ilog_overflowing(self, rhs: Self);
	// fn ilog_saturating(self, rhs: Self);
	// fn ilog_wrapping(self, rhs: Self);
	// fn ilog_carrying(self, rhs: Self);

	// fn ilog2_regular(self, rhs: Self);
	// fn ilog2_checked(self, rhs: Self);
	// unsafe fn ilog2_unchecked(self, rhs: Self);
	// fn ilog2_strict(self, rhs: Self);
	// fn ilog2_overflowing(self, rhs: Self);
	// fn ilog2_saturating(self, rhs: Self);
	// fn ilog2_wrapping(self, rhs: Self);
	// fn ilog2_carrying(self, rhs: Self);

	// fn ilog10_regular(self, rhs: Self);
	// fn ilog10_checked(self, rhs: Self);
	// unsafe fn ilog10_unchecked(self, rhs: Self);
	// fn ilog10_strict(self, rhs: Self);
	// fn ilog10_overflowing(self, rhs: Self);
	// fn ilog10_saturating(self, rhs: Self);
	// fn ilog10_wrapping(self, rhs: Self);
	// fn ilog10_carrying(self, rhs: Self);
}

pub trait WideningUnsignedInt<const BYTES: usize, const WIDENED: usize>: UnsignedInt<BYTES> + sealed::WideningUnsignedInt {
	type Widening: UnsignedInt<WIDENED>;

	fn mul_widening(self, rhs: Self) -> Self::Widening;
}

macro_rules! unsigned_int_trait_impl {
	{ $int:ident $($widening:ident $($rest:ident)*)? } => {
		const _: () = assert!($int::BITS % 8 == 0);

		impl sealed::UnsignedInt for $int {}

		impl UnsignedInt<{ $int::BITS as usize / 8 }> for $int {
			const MIN: $int = $int::MIN;
			const MAX: $int = $int::MAX;
			const BITS: $int = $int::BITS as $int;
			const BYTES: $int = $int::BITS as $int / 8;
			const ZERO: $int = 0;
			const ONE: $int = 1;

			fn from_bool(b: bool) -> $int { b as _ }

			fn add_overflowing(self, rhs: $int) -> ($int, bool) { <$int>::overflowing_add(self, rhs) }
			// TODO: call std?
			fn add_carrying(self, rhs: $int, carry: bool) -> ($int, bool) {
				let (res, carry1) = self.overflowing_add(rhs);
				let (res, carry2) = res.overflowing_add(carry as _);
				(res, carry1 || carry2)
			}

			fn sub_overflowing(self, rhs: $int) -> ($int, bool) { <$int>::overflowing_add(self, rhs) }
			// TODO: call std?
			fn sub_borrowing(self, rhs: $int, borrow: bool) -> ($int, bool) {
				let (res, borrow1) = self.sub_overflowing(rhs);
				let (res, borrow2) = res.sub_overflowing(borrow as _);
				(res, borrow1 || borrow2)
			}

			// TODO: actually unchecked
			unsafe fn mul_unchecked(self, rhs: Self) -> Self { self / rhs }
		}

		$(
			const _: () = assert!($int::BITS as usize * 2 == $widening::BITS as usize);

			impl sealed::WideningUnsignedInt for $int {}
			impl WideningUnsignedInt<{ $int::BITS as usize / 8 }, { $widening::BITS as usize / 8 }> for $int {
				type Widening = $widening;

				fn mul_widening(self, rhs: $int) -> $widening { unsafe { $widening::mul_unchecked(self as $widening, rhs as $widening) } }
			}

			unsigned_int_trait_impl! { $widening $($rest)* }
		)?
	};
}

unsigned_int_trait_impl! { u8 u16 u32 u64 u128 }

/// notouch
mod sealed {
	/// notouch
	pub trait UnsignedInt {}
	/// notouch
	pub trait WideningUnsignedInt {}
}
