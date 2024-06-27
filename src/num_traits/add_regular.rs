use super::Base;

/// Plain addition
pub trait Add: Base + std::ops::Add<Self, Output = Self> {
	#[inline]
	fn add_regular(self, rhs: Self) -> Self {
		self + rhs
	}
}

macro_rules! impl_num_trait_add {
	{ $($num:ident)* } => {
		$(
			impl Add for $num {}
		)*
	}
}

impl_num_trait_add! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}
