#![deny(unconditional_recursion)]

use std::cmp::{ Eq, Ord, PartialEq, PartialOrd };
use std::fmt::{ Debug, Display };
use std::hash::Hash;
use std::iter::{ Sum, Product };
use std::ops;

pub trait UnsignedInt<const BYTES: usize>: Sized + sealed::UnsignedInt
where
	Self: Copy + Debug + Display + Default + Hash,
	Self: PartialEq<Self> + Eq + PartialOrd<Self> + Ord,
	Self: ops::Add<Self, Output = Self> + ops::AddAssign<Self>,
	Self: ops::Sub<Self, Output = Self> + ops::SubAssign<Self>,
	Self: ops::Mul<Self, Output = Self> + ops::MulAssign<Self>,
	Self: ops::Div<Self, Output = Self> + ops::DivAssign<Self>,
	Self: ops::Rem<Self, Output = Self> + ops::RemAssign<Self>,
	Self: ops::Shl<Self, Output = Self> + ops::ShlAssign<Self>,
	Self: ops::Shr<Self, Output = Self> + ops::ShrAssign<Self>,
	Self: ops::Not<Output = Self>,
	Self: ops::BitAnd<Self, Output = Self> + ops::BitAndAssign<Self>,
	Self: ops::BitOr<Self, Output = Self> + ops::BitOrAssign<Self>,
	Self: ops::BitXor<Self, Output = Self> + ops::BitXorAssign<Self>,
	Self: Sum<Self> + Product<Self>
{
	/// The smallest value that can be represented by this int type
	const MIN: Self;
	/// The largest value that can be represented by this int type
	const MAX: Self;
	/// The size of this integer type in bits
	const BITS: Self;
	/// The size of this integer type in bytes
	// TODO: this / generic param ehh weird funny
	const BYTES: Self;
	/// Zero
	const ZERO: Self;
	/// One
	const ONE: Self;

	/// Cast a boolean into `Self`
	///
	/// This returns `0` if `b` is `false`, and `1` if `b` is `true`
	fn from_bool(b: bool) -> Self;

	// reg checked unchecked strict overflowing saturating wrapping carrying
	// add sub mul div
	// rem shl shr pow
	// neg ilog ilog2 ilog10

	// fn add_regular(self, rhs: Self);
	// fn add_checked(self, rhs: Self);
	// unsafe fn add_unchecked(self, rhs: Self);
	// fn add_strict(self, rhs: Self);
	fn add_overflowing(self, rhs: Self) -> (Self, bool);
	// fn add_saturating(self, rhs: Self);
	// fn add_wrapping(self, rhs: Self);
	fn add_carrying(self, rhs: Self, carry: bool) -> (Self, bool);

	// fn sub_regular(self, rhs: Self);
	// fn sub_checked(self, rhs: Self);
	// unsafe fn sub_unchecked(self, rhs: Self);
	// fn sub_strict(self, rhs: Self);
	fn sub_overflowing(self, rhs: Self) -> (Self, bool);
	// fn sub_saturating(self, rhs: Self);
	// fn sub_wrapping(self, rhs: Self);
	fn sub_borrowing(self, rhs: Self, borrow: bool) -> (Self, bool);

	// fn mul_regular(self, rhs: Self);
	// fn mul_checked(self, rhs: Self);
	unsafe fn mul_unchecked(self, rhs: Self) -> Self;
	// fn mul_strict(self, rhs: Self);
	// fn mul_overflowing(self, rhs: Self);
	// fn mul_saturating(self, rhs: Self);
	// fn mul_wrapping(self, rhs: Self);
	// fn mul_carrying(self, rhs: Self);

	// fn div_regular(self, rhs: Self);
	// fn div_checked(self, rhs: Self);
	// unsafe fn div_unchecked(self, rhs: Self);
	// fn div_strict(self, rhs: Self);
	// fn div_overflowing(self, rhs: Self);
	// fn div_saturating(self, rhs: Self);
	// fn div_wrapping(self, rhs: Self);
	// fn div_carrying(self, rhs: Self);

	// fn rem_regular(self, rhs: Self);
	// fn rem_checked(self, rhs: Self);
	// unsafe fn rem_unchecked(self, rhs: Self);
	// fn rem_strict(self, rhs: Self);
	// fn rem_overflowing(self, rhs: Self);
	// fn rem_saturating(self, rhs: Self);
	// fn rem_wrapping(self, rhs: Self);
	// fn rem_carrying(self, rhs: Self);

	// fn shl_regular(self, rhs: Self);
	// fn shl_checked(self, rhs: Self);
	// unsafe fn shl_unchecked(self, rhs: Self);
	// fn shl_strict(self, rhs: Self);
	// fn shl_overflowing(self, rhs: Self);
	// fn shl_saturating(self, rhs: Self);
	// fn shl_wrapping(self, rhs: Self);
	// fn shl_carrying(self, rhs: Self);

	// fn shr_regular(self, rhs: Self);
	// fn shr_checked(self, rhs: Self);
	// unsafe fn shr_unchecked(self, rhs: Self);
	// fn shr_strict(self, rhs: Self);
	// fn shr_overflowing(self, rhs: Self);
	// fn shr_saturating(self, rhs: Self);
	// fn shr_wrapping(self, rhs: Self);
	// fn shr_carrying(self, rhs: Self);

	// fn pow_regular(self, rhs: Self);
	// fn pow_checked(self, rhs: Self);
	// unsafe fn pow_unchecked(self, rhs: Self);
	// fn pow_strict(self, rhs: Self);
	// fn pow_overflowing(self, rhs: Self);
	// fn pow_saturating(self, rhs: Self);
	// fn pow_wrapping(self, rhs: Self);
	// fn pow_carrying(self, rhs: Self);

	// fn neg_regular(self, rhs: Self);
	// fn neg_checked(self, rhs: Self);
	// unsafe fn neg_unchecked(self, rhs: Self);
	// fn neg_strict(self, rhs: Self);
	// fn neg_overflowing(self, rhs: Self);
	// fn neg_saturating(self, rhs: Self);
	// fn neg_wrapping(self, rhs: Self);
	// fn neg_carrying(self, rhs: Self);

	// fn ilog_regular(self, rhs: Self);
	// fn ilog_checked(self, rhs: Self);
	// unsafe fn ilog_unchecked(self, rhs: Self);
	// fn ilog_strict(self, rhs: Self);
	// fn ilog_overflowing(self, rhs: Self);
	// fn ilog_saturating(self, rhs: Self);
	// fn ilog_wrapping(self, rhs: Self);
	// fn ilog_carrying(self, rhs: Self);

	// fn ilog2_regular(self, rhs: Self);
	// fn ilog2_checked(self, rhs: Self);
	// unsafe fn ilog2_unchecked(self, rhs: Self);
	// fn ilog2_strict(self, rhs: Self);
	// fn ilog2_overflowing(self, rhs: Self);
	// fn ilog2_saturating(self, rhs: Self);
	// fn ilog2_wrapping(self, rhs: Self);
	// fn ilog2_carrying(self, rhs: Self);

	// fn ilog10_regular(self, rhs: Self);
	// fn ilog10_checked(self, rhs: Self);
	// unsafe fn ilog10_unchecked(self, rhs: Self);
	// fn ilog10_strict(self, rhs: Self);
	// fn ilog10_overflowing(self, rhs: Self);
	// fn ilog10_saturating(self, rhs: Self);
	// fn ilog10_wrapping(self, rhs: Self);
	// fn ilog10_carrying(self, rhs: Self);
}

pub trait WideningUnsignedInt<const BYTES: usize, const WIDENED: usize>: UnsignedInt<BYTES> + sealed::WideningUnsignedInt {
	type Widening: UnsignedInt<WIDENED>;

	/// (l, h) (little endian)
	fn split_wide(wide: Self::Widening) -> (Self, Self);

	fn mul_widening_nosplit(self, rhs: Self) -> Self::Widening;
}

macro_rules! unsigned_int_trait_impl {
	{ $int:ident $($widening:ident $($rest:ident)*)? } => {
		const _: () = assert!($int::BITS % 8 == 0);

		impl sealed::UnsignedInt for $int {}

		impl UnsignedInt<{ $int::BITS as usize / 8 }> for $int {
			const MIN: $int = $int::MIN;
			const MAX: $int = $int::MAX;
			const BITS: $int = $int::BITS as $int;
			const BYTES: $int = $int::BITS as $int / 8;
			const ZERO: $int = 0;
			const ONE: $int = 1;

			fn from_bool(b: bool) -> $int { b as _ }

			fn add_overflowing(self, rhs: $int) -> ($int, bool) { <$int>::overflowing_add(self, rhs) }
			// TODO: call std?
			fn add_carrying(self, rhs: $int, carry: bool) -> ($int, bool) {
				let (res, carry1) = self.overflowing_add(rhs);
				let (res, carry2) = res.overflowing_add(carry as _);
				(res, carry1 || carry2)
			}

			fn sub_overflowing(self, rhs: $int) -> ($int, bool) { <$int>::overflowing_sub(self, rhs) }
			// TODO: call std?
			fn sub_borrowing(self, rhs: $int, borrow: bool) -> ($int, bool) {
				let (res, borrow1) = self.sub_overflowing(rhs);
				let (res, borrow2) = res.sub_overflowing(borrow as _);
				(res, borrow1 || borrow2)
			}

			// TODO: actually unchecked
			unsafe fn mul_unchecked(self, rhs: $int) -> $int { self * rhs }
		}

		$(
			const _: () = assert!($int::BITS as usize * 2 == $widening::BITS as usize);

			impl sealed::WideningUnsignedInt for $int {}
			impl WideningUnsignedInt<{ $int::BITS as usize / 8 }, { $widening::BITS as usize / 8 }> for $int {
				type Widening = $widening;

				fn split_wide(wide: $widening) -> ($int, $int) { (wide as $int, (wide >> $int::BITS) as $int) }

				fn mul_widening_nosplit(self, rhs: $int) -> $widening { unsafe { $widening::mul_unchecked(self as $widening, rhs as $widening) } }
			}

			unsigned_int_trait_impl! { $widening $($rest)* }
		)?
	};
}

unsigned_int_trait_impl! { u8 u16 u32 u64 u128 }

/// notouch
mod sealed {
	/// notouch
	pub trait UnsignedInt {}
	/// notouch
	pub trait WideningUnsignedInt {}
}

mod base;
pub use base::Base;
mod add_regular;
pub use add_regular::Add;
mod add_checked;
pub use add_checked::AddChecked;
mod add_unchecked;
pub use add_unchecked::AddUnchecked;
mod add_strict;
pub use add_strict::AddStrict;
mod add_overflowing;
pub use add_overflowing::AddOverflowing;
mod add_saturating;
pub use add_saturating::AddSaturating;
mod add_wrapping;
pub use add_wrapping::AddWrapping;
mod add_carrying;
pub use add_carrying::AddCarrying;
mod sub_regular;
pub use sub_regular::Sub;
mod mul_regular;
pub use mul_regular::Mul;
mod div_regular;
pub use div_regular::Div;
mod rem_regular;
pub use rem_regular::Rem;
mod shl_regular;
pub use shl_regular::Shl;
mod shr_regular;
pub use shr_regular::Shr;
mod pow_regular;
pub use pow_regular::Pow;
mod neg_regular;
pub use neg_regular::Neg;
mod not_regular;
pub use not_regular::Not;
mod and_regular;
pub use and_regular::And;
mod or_regular;
pub use or_regular::Or;
mod xor_regular;
pub use xor_regular::Xor;
mod array_conversions;
pub use array_conversions::ArrayConversions;

// // TODO: ilog/2/10 sum(?) product(?)
