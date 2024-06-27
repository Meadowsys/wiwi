use super::Base;

/// Trait for number types that support overflowing addition
pub trait AddOverflowing: Sized + Base {
	fn add_overflowing(self, rhs: Self) -> (Self, bool);
}
