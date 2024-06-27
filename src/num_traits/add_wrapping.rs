use super::Base;

/// Wrapping addition
pub trait AddWrapping: Base {
	fn add_wrapping(self, rhs: Self) -> Self;
}
