use super::Base;

/// Trait for number types that support exponent
pub trait PowRegular: Sized + Base {
	fn pow_regular(self, exp: Self) -> Self;
}
