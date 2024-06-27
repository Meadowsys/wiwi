use super::Base;

/// Strict addition
pub trait AddStrict: Base {
	fn add_strict(self, rhs: Self) -> Self;
}
