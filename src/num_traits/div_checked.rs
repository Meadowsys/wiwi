use super::Div;

pub trait DivChecked: Div {
	fn div_checked(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_num_trait_div_checked {
	{ $($num:ident)* } => {
		$(
			impl DivChecked for $num {
				#[inline]
				fn div_checked(self, rhs: $num) -> Option<$num> {
					$num::checked_div(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_div_checked! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
