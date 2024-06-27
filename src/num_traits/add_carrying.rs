use super::Base;

/// Trait for number types that support carrying addition
pub trait AddCarrying: Sized + Base {
	fn add_carrying(self, rhs: Self, carry: bool) -> (Self, bool);
}
