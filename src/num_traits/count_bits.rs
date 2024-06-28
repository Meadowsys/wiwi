use super::Base;

pub trait CountBits {
	fn count_zeros(self) -> Self;
	fn count_ones(self) -> Self;
}
