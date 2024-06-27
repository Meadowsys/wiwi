use super::Base;

/// Bitwise OR
pub trait Or: Base + std::ops::BitOr<Self, Output = Self> {
	#[inline]
	fn or_regular(self, rhs: Self) -> Self {
		self | rhs
	}
}
