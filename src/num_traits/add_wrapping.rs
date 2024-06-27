use super::Base;

/// Wrapping addition
pub trait AddWrapping: Sized + Base {
	fn add_wrapping(self, rhs: Self) -> Self;
}
