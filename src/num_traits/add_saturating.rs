use super::Base;

/// Saturating addition
pub trait AddSaturating: Sized + Base {
	fn add_saturating(self, rhs: Self) -> Self;
}
