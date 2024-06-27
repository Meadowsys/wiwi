use super::Base;

/// Checked addition
pub trait AddChecked: Sized + Base {
	fn add_checked(self, rhs: Self) -> Option<Self>;
}
