use super::Base;

/// Trait for number types that support checked addition
pub trait AddChecked: Sized + Base {
	fn add_checked(self, rhs: Self) -> Option<Self>;
}
