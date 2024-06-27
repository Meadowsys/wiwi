use super::Base;

/// Plain multiplication
pub trait Mul: Sized + Base + std::ops::Mul<Self, Output = Self> {
	#[inline]
	fn mul_regular(self, rhs: Self) -> Self {
		self * rhs
	}
}
