use super::Base;

/// Trait for number types that support division
pub trait DivRegular: Sized + Base + std::ops::Div<Self, Output = Self> {
	#[inline]
	fn div_regular(self, rhs: Self) -> Self {
		self / rhs
	}
}
