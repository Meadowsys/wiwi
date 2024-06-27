use super::Base;

/// Trait for number types that support the bitwise AND operator
pub trait AndRegular: Sized + Base + std::ops::BitAnd<Self, Output = Self> {
	#[inline]
	fn and_regular(self, rhs: Self) -> Self {
		self & rhs
	}
}
