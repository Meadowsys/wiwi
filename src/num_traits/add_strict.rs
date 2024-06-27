use super::Base;

/// Trait for number types that support strict addition
pub trait AddStrict: Sized + Base {
	fn add_strict(self, rhs: Self) -> Self;
}
