use super::Base;

/// Trait for number types that support addition
pub trait AddRegular: Sized + Base + std::ops::Add<Self, Output = Self> {
	#[inline]
	fn add_regular(self, rhs: Self) -> Self {
		self + rhs
	}
}
