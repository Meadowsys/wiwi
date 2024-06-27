use super::Base;

/// Right shift
pub trait Shr: Sized + Base + std::ops::Shr<Self, Output = Self> {
	#[inline]
	fn shr_regular(self, rhs: Self) -> Self {
		self >> rhs
	}
}
