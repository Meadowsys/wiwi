use super::Base;

/// Trait for number types that support multiplication
pub trait MulRegular: Sized + Base + std::ops::Mul<Self, Output = Self> {
	#[inline]
	fn mul_regular(self, rhs: Self) -> Self {
		self * rhs
	}
}
