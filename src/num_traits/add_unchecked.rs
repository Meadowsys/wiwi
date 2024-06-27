use super::Base;

/// Trait for number types that support unchecked addition
pub trait AddUnchecked: Sized + Base {
	unsafe fn add_unchecked(self, rhs: Self) -> Self;
}
