use super::Div;

/// Integer division
pub trait DivInt: Div {
	#[inline]
	fn div_int(self, rhs: Self) -> Self {
		self / rhs
	}
}

macro_rules! impl_num_trait_div_int {
	{ $($num:ident)* } => {
		$(
			impl DivInt for $num {}
		)*
	}
}

impl_num_trait_div_int! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
