pub trait Int<const BYTES: usize>: Sized + Clone {
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

	// TODO: from_be
	// TODO: from_le
	// TODO: to_be
	// TODO: to_le
	// TODO: from_be_bytes
	// TODO: from_le_bytes
	// TODO: from_ne_bytes
	// TODO: to_be_bytes
	// TODO: to_le_bytes
	// TODO: to_ne_bytes

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
}

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
					@fn(&self) -> Self
					count_ones
					count_zeros
					leading_zeros
					trailing_zeros
					leading_ones
					trailing_ones
				}

				int_trait_impl! {
					@fn(&mut self, &Self)
					rotate_left
					rotate_right
				}
			}
		)*
	};

	{ @fn(&self) -> Self $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(&self) -> Self {
				Self::$fn_name(*self) as _
			}
		)*
	};

	{ @fn(&mut self, &Self) $($fn_name:ident)* } => {
		$(
			#[inline(always)]
			fn $fn_name(&mut self, arg: &Self) {
				*self = Self::$fn_name(*self, *arg as _) as _
			}
		)*
	};
}

int_trait_impl! {
	u8 u16 u32 u64 u128
	i8 i16 i32 i64 i128
}
