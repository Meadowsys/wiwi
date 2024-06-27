use super::Base;

/// Saturating addition
pub trait AddSaturating: Base {
	fn add_saturating(self, rhs: Self) -> Self;
}
