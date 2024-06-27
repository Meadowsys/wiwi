use super::Base;

/// Exponentiation
pub trait Pow: Sized + Base {
	fn pow_regular(self, exp: Self) -> Self;
}
