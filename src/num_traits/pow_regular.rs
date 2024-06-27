use super::Base;

/// Exponentiation
pub trait Pow: Base {
	fn pow_regular(self, exp: Self) -> Self;
}
