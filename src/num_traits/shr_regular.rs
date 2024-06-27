use super::Base;

/// Trait for number types that support right shift
pub trait ShrRegular: Sized + Base + std::ops::Shr<Self, Output = Self> {
	#[inline]
	fn shr_regular(self, rhs: Self) -> Self {
		self >> rhs
	}
}
