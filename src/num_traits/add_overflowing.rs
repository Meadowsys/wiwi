use super::Base;

/// Overflowing addition
pub trait AddOverflowing: Base {
	fn add_overflowing(self, rhs: Self) -> (Self, bool);
}
