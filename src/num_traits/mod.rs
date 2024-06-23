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
}

pub trait Int<const BYTES: usize>: Sized + Copy {
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
}

impl<const BYTES: usize, T: Int<BYTES>> BigInt<BYTES> for T {
	const MIN: Self = <Self as Int<BYTES>>::MIN;
	const MAX: Self = <Self as Int<BYTES>>::MAX;
	const BITS: Self = <Self as Int<BYTES>>::BITS;
	const BYTES: Self = <Self as Int<BYTES>>::BYTES;

	fn count_ones(&self) -> Self { Int::<BYTES>::count_ones(*self) }
	fn count_zeros(&self) -> Self { Int::<BYTES>::count_zeros(*self) }
	fn leading_zeros(&self) -> Self { Int::<BYTES>::leading_zeros(*self) }
	fn trailing_zeros(&self) -> Self { Int::<BYTES>::trailing_zeros(*self) }
	fn leading_ones(&self) -> Self { Int::<BYTES>::leading_ones(*self) }
	fn trailing_ones(&self) -> Self { Int::<BYTES>::trailing_ones(*self) }

	fn rotate_left(&mut self, n: &Self) { *self = Int::<BYTES>::rotate_left(*self, *n) }
	fn rotate_right(&mut self, n: &Self) { *self = Int::<BYTES>::rotate_right(*self, *n) }

	fn from_be_ref(x: &Self) -> Self { Int::<BYTES>::from_be(*x) }
	fn from_le_ref(x: &Self) -> Self { Int::<BYTES>::from_le(*x) }
	fn from_be(x: Self) -> Self { Int::<BYTES>::from_be(x) }
	fn from_le(x: Self) -> Self { Int::<BYTES>::from_le(x) }

	fn to_be(&self) -> Self { Int::<BYTES>::to_be(*self) }
	fn to_le(&self) -> Self { Int::<BYTES>::to_le(*self) }
	fn into_be(self) -> Self { Int::<BYTES>::to_be(self) }
	fn into_le(self) -> Self { Int::<BYTES>::to_le(self) }

	fn from_be_bytes_ref(bytes: &[u8; BYTES]) -> Self { Int::<BYTES>::from_be_bytes(*bytes) }
	fn from_le_bytes_ref(bytes: &[u8; BYTES]) -> Self { Int::<BYTES>::from_le_bytes(*bytes) }
	fn from_ne_bytes_ref(bytes: &[u8; BYTES]) -> Self { Int::<BYTES>::from_ne_bytes(*bytes) }
	fn from_be_bytes(bytes: [u8; BYTES]) -> Self { Int::<BYTES>::from_be_bytes(bytes) }
	fn from_le_bytes(bytes: [u8; BYTES]) -> Self { Int::<BYTES>::from_le_bytes(bytes) }
	fn from_ne_bytes(bytes: [u8; BYTES]) -> Self { Int::<BYTES>::from_ne_bytes(bytes) }

	fn to_be_bytes(&self) -> [u8; BYTES] { Int::<BYTES>::to_be_bytes(*self) }
	fn to_le_bytes(&self) -> [u8; BYTES] { Int::<BYTES>::to_le_bytes(*self) }
	fn to_ne_bytes(&self) -> [u8; BYTES] { Int::<BYTES>::to_ne_bytes(*self) }
	fn into_be_bytes(self) -> [u8; BYTES] { Int::<BYTES>::to_be_bytes(self) }
	fn into_le_bytes(self) -> [u8; BYTES] { Int::<BYTES>::to_le_bytes(self) }
	fn into_ne_bytes(self) -> [u8; BYTES] { Int::<BYTES>::to_ne_bytes(self) }
}

// TODO: swap_bytes
// TODO: reverse_bits
// TODO: checked_add
// TODO: strict_add
// TODO: unchecked_add
// TODO: checked_add_signed
// TODO: strict_add_signed
// TODO: checked_sub
// TODO: strict_sub
// TODO: unchecked_sub
// TODO: checked_mul
// TODO: strict_mul
// TODO: unchecked_mul
// TODO: checked_div
// TODO: strict_div
// TODO: checked_div_euclid
// TODO: strict_div_euclid
// TODO: checked_rem
// TODO: strict_rem
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
// TODO: checked_shl
// TODO: strict_shl
// TODO: unchecked_shl
// TODO: checked_shr
// TODO: strict_shr
// TODO: unchecked_shr
// TODO: checked_pow
// TODO: strict_pow
// TODO: saturating_add
// TODO: saturating_add_signed
// TODO: saturating_sub
// TODO: saturating_mul
// TODO: saturating_div
// TODO: saturating_pow
// TODO: wrapping_add
// TODO: wrapping_add_signed
// TODO: wrapping_sub
// TODO: wrapping_mul
// TODO: wrapping_div
// TODO: wrapping_div_euclid
// TODO: wrapping_rem
// TODO: wrapping_rem_euclid
// TODO: wrapping_neg
// TODO: wrapping_shl
// TODO: wrapping_shr
// TODO: wrapping_pow
// TODO: overflowing_add
// TODO: carrying_add
// TODO: overflowing_add_signed
// TODO: overflowing_sub
// TODO: borrowing_sub
// TODO: abs_diff
// TODO: overflowing_mul
// TODO: overflowing_div
// TODO: overflowing_div_euclid
// TODO: overflowing_rem
// TODO: overflowing_rem_euclid
// TODO: overflowing_neg
// TODO: overflowing_shl
// TODO: overflowing_shr
// TODO: overflowing_pow
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
	{ $($int:ident)* } => {
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
}

int_trait_impl! {
	u8 u16 u32 u64 u128
	i8 i16 i32 i64 i128
}
