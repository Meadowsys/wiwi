use super::Base;

/// Modulo (the remainder operator)
pub trait Rem: Base + std::ops::Rem<Self, Output = Self> {
	#[inline]
	fn rem_regular(self, rhs: Self) -> Self {
		self % rhs
	}
}
