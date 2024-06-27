use super::Base;

/// Plain subtraction
pub trait Sub: Base + std::ops::Sub<Self, Output = Self> {
	#[inline]
	fn sub_regular(self, rhs: Self) -> Self {
		self - rhs
	}
}

macro_rules! impl_num_trait_sub {
	{ $($num:ident)* } => {
		$(
			impl Sub for $num {}
		)*
	}
}

impl_num_trait_sub! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}
