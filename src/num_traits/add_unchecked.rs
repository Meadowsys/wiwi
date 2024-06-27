use super::Base;

/// Unchecked addition
pub trait AddUnchecked: Sized + Base {
	unsafe fn add_unchecked(self, rhs: Self) -> Self;
}
