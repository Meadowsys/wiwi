use super::Base;

/// Trait for number types that support the bitwise OR operator
pub trait OrRegular: Sized + Base + std::ops::BitOr<Self, Output = Self> {
	#[inline]
	fn or_regular(self, rhs: Self) -> Self {
		self | rhs
	}
}
