use super::Base;

/// Plain subtraction
pub trait Sub: Sized + Base + std::ops::Sub<Self, Output = Self> {
	#[inline]
	fn sub_regular(self, rhs: Self) -> Self {
		self - rhs
	}
}
