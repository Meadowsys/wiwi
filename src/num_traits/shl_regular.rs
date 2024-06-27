use super::Base;

/// Left shift
pub trait Shl: Base + std::ops::Shl<Self, Output = Self> {
	#[inline]
	fn shl_regular(self, rhs: Self) -> Self {
		self << rhs
	}
}
