use super::Base;

/// Unchecked addition
pub trait AddUnchecked: Base {
	unsafe fn add_unchecked(self, rhs: Self) -> Self;
}
