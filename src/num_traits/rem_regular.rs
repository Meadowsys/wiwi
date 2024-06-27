use super::Base;

/// Trait for number types that support modulo, or the remainder operator
pub trait RemRegular: Sized + Base + std::ops::Rem<Self, Output = Self> {
	#[inline]
	fn rem_regular(self, rhs: Self) -> Self {
		self % rhs
	}
}
