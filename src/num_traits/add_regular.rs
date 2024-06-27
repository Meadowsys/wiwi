use super::Base;

/// Plain addition
pub trait Add: Sized + Base + std::ops::Add<Self, Output = Self> {
	#[inline]
	fn add_regular(self, rhs: Self) -> Self {
		self + rhs
	}
}
