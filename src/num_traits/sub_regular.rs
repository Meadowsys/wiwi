use super::Base;

/// Trait for number types that support subtraction
pub trait SubRegular: Sized + Base + std::ops::Sub<Self, Output = Self> {
	#[inline]
	fn sub_regular(self, rhs: Self) -> Self {
		self - rhs
	}
}
