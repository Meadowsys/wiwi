use super::Base;

/// Bitwise OR
pub trait Or: Sized + Base + std::ops::BitOr<Self, Output = Self> {
	#[inline]
	fn or_regular(self, rhs: Self) -> Self {
		self | rhs
	}
}
