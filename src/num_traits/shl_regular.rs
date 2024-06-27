use super::Base;

/// Trait for number types that support left shift
pub trait ShlRegular: Sized + Base + std::ops::Shl<Self, Output = Self> {
	#[inline]
	fn shl_regular(self, rhs: Self) -> Self {
		self << rhs
	}
}
