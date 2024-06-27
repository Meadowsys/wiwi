use super::Base;

/// Trait for number types that support saturating addition
pub trait AddSaturating: Sized + Base {
	fn add_saturating(self, rhs: Self) -> Self;
}
