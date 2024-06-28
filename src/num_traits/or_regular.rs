use super::Base;

/// Bitwise OR
pub trait Or: Base + std::ops::BitOr<Self, Output = Self> {
	#[inline]
	fn or_regular(self, rhs: Self) -> Self {
		self | rhs
	}
}

macro_rules! impl_num_trait_or_checked {
	{ $($num:ident)* } => {
		$(
			impl Or for $num {}
		)*
	}
}

impl_num_trait_or_checked! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
