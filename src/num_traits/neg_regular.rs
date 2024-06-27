use super::Base;

/// Negation
pub trait Neg: Base + std::ops::Neg<Output = Self> {
	#[inline]
	fn neg_regular(self) -> Self {
		-self
	}
}
