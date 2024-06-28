use super::Add;

/// Checked addition
pub trait AddChecked: Add {
	fn add_checked(self, rhs: Self) -> Option<Self>;
}
