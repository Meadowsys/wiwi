use super::Base;

/// Trait for number types that support the bitwise XOR operator
pub trait XorRegular: Sized + Base + std::ops::BitXor<Self, Output = Self> {
	#[inline]
	fn xor_regular(self, rhs: Self) -> Self {
		self ^ rhs
	}
}
