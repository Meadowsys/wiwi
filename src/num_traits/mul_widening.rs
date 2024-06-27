use super::{ MulUnchecked, Widening };

/// Widening multiplication
pub trait MulWidening: MulUnchecked + Widening {
	/// Performs widening multiplication, returning the result in two parts:
	/// `(low, high)`
	///
	/// This will never overflow
	///
	/// # Examples
	///
	/// TODO
	#[inline]
	fn mul_widening(self, rhs: Self) -> (Self, Self) {
		let res = Self::mul_widening_no_split(self, rhs);
		Self::split(res)
	}

	/// Performs widening multiplication, returning the result as the widened part
	///
	/// This will never overflow
	///
	/// # Examples
	///
	/// TODO
	fn mul_widening_no_split(self, rhs: Self) -> Self::Wide;
}

macro_rules! impl_num_trait_mul_widening {
	{ $($num:ident)* } => {
		$(
			impl MulWidening for $num {
				#[inline]
				fn mul_widening_no_split(self, rhs: $num) -> <$num as Widening>::Wide {
					unsafe { Self::Wide::mul_unchecked(Self::widen(self), Self::widen(rhs)) }
				}
			}
		)*
	}
}

impl_num_trait_mul_widening! {
	u8 u16 u32 u64 usize
	i8 i16 i32 i64 isize
}
