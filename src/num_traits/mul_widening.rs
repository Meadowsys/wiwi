use super::{ MulUnchecked, Widening };

/// Widening multiplication
pub trait MulWidening: MulUnchecked {
	/// Performs widening multiplication, returning the result in two parts:
	/// `(low, high)`
	///
	/// This will never overflow
	///
	/// # Examples
	///
	/// TODO
	fn mul_widening(self, rhs: Self) -> (Self, Self);
}

macro_rules! impl_num_trait_mul_widening {
	{ $num:ident $wide:ident $($next_wide:ident $($rest:ident)*)? } => {
		impl MulWidening for $num {
			#[inline]
			fn mul_widening(self, rhs: $num) -> ($num, $num) {
				let widened = unsafe { $wide::mul_unchecked(Self::widen(self), Self::widen(rhs)) };
				<$num as Widening<$wide>>::split(widened)
			}
		}

		$(impl_num_trait_mul_widening! { $wide $next_wide $($rest)* })?
	}
}

impl_num_trait_mul_widening! {
	u8 u16 u32 u64 u128
}

#[cfg(target_pointer_width = "64")]
impl_num_trait_mul_widening! { usize u128 }

#[cfg(target_pointer_width = "32")]
impl_num_trait_mul_widening! { usize u64 }

#[cfg(target_pointer_width = "16")]
impl_num_trait_mul_widening! { usize u32 }
