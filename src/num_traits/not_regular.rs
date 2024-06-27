use super::Base;

/// Trait for number types that support the bitwise NOT operator
pub trait NotRegular: Sized + Base + std::ops::Not<Output = Self> {
	#[inline]
	fn not_regular(self) -> Self {
		!self
	}
}
