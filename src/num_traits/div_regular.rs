use super::Base;

/// Plain division
pub trait Div: Sized + Base + std::ops::Div<Self, Output = Self> {
	#[inline]
	fn div_regular(self, rhs: Self) -> Self {
		self / rhs
	}
}
