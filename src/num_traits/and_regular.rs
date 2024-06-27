use super::Base;

/// Bitwise AND
pub trait And: Sized + Base + std::ops::BitAnd<Self, Output = Self> {
	#[inline]
	fn and_regular(self, rhs: Self) -> Self {
		self & rhs
	}
}
