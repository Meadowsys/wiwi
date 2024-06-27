use super::Base;

/// Strict addition
pub trait AddStrict: Sized + Base {
	fn add_strict(self, rhs: Self) -> Self;
}
