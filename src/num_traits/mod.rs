#![deny(unconditional_recursion)]

use std::cmp::{ Eq, Ord, PartialEq, PartialOrd };
use std::fmt::{ Debug, Display };
use std::hash::Hash;
use std::iter::{ Sum, Product };
use std::ops;

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
	const MIN: Self;
	const MAX: Self;
	const BITS: Self;
	// TODO: this / generic param ehh weird funny
	const BYTES: Self;

	fn count_ones(self) -> Self;
	fn count_zeros(self) -> Self;
	fn leading_zeros(self) -> Self;
	fn trailing_zeros(self) -> Self;
	fn leading_ones(self) -> Self;
	fn trailing_ones(self) -> Self;
	fn rotate_left(self, n: Self) -> Self;
	fn rotate_right(self, n: Self) -> Self;
	fn from_be(x: Self) -> Self;
	fn from_le(x: Self) -> Self;
	fn to_be(self) -> Self;
	fn to_le(self) -> Self;

	fn from_be_bytes(bytes: [u8; BYTES]) -> Self;
	fn from_le_bytes(bytes: [u8; BYTES]) -> Self;
	fn from_ne_bytes(bytes: [u8; BYTES]) -> Self;

	fn to_be_bytes(self) -> [u8; BYTES];
	fn to_le_bytes(self) -> [u8; BYTES];
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

pub trait BigInt<const BYTES: usize>: Sized {
	const MIN: Self;
	const MAX: Self;
	const BITS: Self;
	// TODO: this / generic param ehh weird funny
	const BYTES: Self;

	fn count_ones(&self) -> Self;
	fn count_zeros(&self) -> Self;
	fn leading_zeros(&self) -> Self;
	fn trailing_zeros(&self) -> Self;
	fn leading_ones(&self) -> Self;
	fn trailing_ones(&self) -> Self;

	fn rotate_left(&mut self, n: &Self);
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
