use super::Base;

/// Bitwise XOR
pub trait Xor: Base + std::ops::BitXor<Self, Output = Self> {
	#[inline]
	fn xor_regular(self, rhs: Self) -> Self {
		self ^ rhs
	}
}
