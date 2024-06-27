use super::Base;

/// Trait for number types that support wrapping addition
pub trait AddWrapping: Sized + Base {
	fn add_wrapping(self, rhs: Self) -> Self;
}
