use super::Base;

/// Bitwise NOT
pub trait Not: Sized + Base + std::ops::Not<Output = Self> {
	#[inline]
	fn not_regular(self) -> Self {
		!self
	}
}