use super::Base;

/// Carrying addition
pub trait AddCarrying: Sized + Base {
	fn add_carrying(self, rhs: Self, carry: bool) -> (Self, bool);
}
