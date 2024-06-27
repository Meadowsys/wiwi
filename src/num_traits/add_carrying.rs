use super::Base;

/// Carrying addition
pub trait AddCarrying: Base {
	fn add_carrying(self, rhs: Self, carry: bool) -> (Self, bool);
}
