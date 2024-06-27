use super::Base;

/// Bitwise NOT
pub trait Not: Base + std::ops::Not<Output = Self> {
	#[inline]
	fn not_regular(self) -> Self {
		!self
	}
}
