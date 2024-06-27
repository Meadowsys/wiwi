use super::Base;

/// Overflowing addition
pub trait AddOverflowing: Sized + Base {
	fn add_overflowing(self, rhs: Self) -> (Self, bool);
}
