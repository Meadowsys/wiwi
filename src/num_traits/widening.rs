use super::Base;

/// Numbers that can be "widened" to another number type that's double it's size
pub trait Widening: Base {
	/// The "widened" type, the type that's twice the size as this one
	type Wide: Base;

	/// Extend `self` out into the widened type (without changing its value)
	fn widen(self) -> Self::Wide;

	/// Splits the wide integer into its lower and upper parts
	///
	/// The returned value is (low, high), ie. first element is the lower half,
	/// and the second element is the upper half, ie. little endian order.
	fn split(wide: Self::Wide) -> (Self, Self);
}

macro_rules! impl_num_trait_widening {
	{ $num:ident $wide:ident $($next_wide:ident $($rest:ident)*)? } => {
		impl Widening for $num {
			type Wide = $wide;
			#[inline(always)]
			fn widen(self) -> $wide { self as _ }
			#[inline]
			fn split(wide: $wide) -> ($num, $num) { (wide as _, (wide >> <$num as Base>::BITS) as _) }
		}
		$(impl_num_trait_widening! { $wide $next_wide $($rest)* })?
	}
}

impl_num_trait_widening! { u8 u16 u32 u64 u128 }
impl_num_trait_widening! { i8 i16 i32 i64 i128 }

#[cfg(target_pointer_width = "64")]
impl_num_trait_widening! { usize u128 }
#[cfg(target_pointer_width = "64")]
impl_num_trait_widening! { isize i128 }

#[cfg(target_pointer_width = "32")]
impl_num_trait_widening! { usize u64 }
#[cfg(target_pointer_width = "32")]
impl_num_trait_widening! { isize i64 }

#[cfg(target_pointer_width = "16")]
impl_num_trait_widening! { usize u32 }
#[cfg(target_pointer_width = "16")]
impl_num_trait_widening! { isize i32 }
