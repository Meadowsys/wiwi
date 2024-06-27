use super::Base;

/// Trait for number types that support negating
pub trait NegRegular: Sized + Base + std::ops::Neg<Output = Self> {
	#[inline]
	fn neg_regular(self) -> Self {
		-self
	}
}
