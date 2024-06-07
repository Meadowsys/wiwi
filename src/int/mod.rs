use std::mem::MaybeUninit;
use std::ops::{
	Add,
	Sub,
	Mul,
	Div,
	Neg,
	Rem,
	AddAssign,
	SubAssign,
	MulAssign,
	DivAssign,
	RemAssign,
	BitAnd,
	BitOr,
	BitXor,
	Not,
	Shl,
	Shr,
	BitAndAssign,
	BitOrAssign,
	BitXorAssign,
	ShlAssign,
	ShrAssign
};

pub trait UnsignedBigint<P, const N: usize>
where
	P: Part,
	Self: Clone,
	// Self: From<u8>,
	// Self: From<u16>,
	// Self: From<u32>,
	// Self: From<u64>,
	// Self: From<u128>,
{
	const MIN: Self;
	const MAX: Self;
	const BITS: u32;

	fn into_le_parts(self) -> [P; N];
	fn from_le_parts(parts: [P; N]) -> Self;

	unsafe fn add_unchecked(self, rhs: Self) -> Self {
		let parts_self = self.into_le_parts();
		let parts_self = &parts_self as *const P;

		let parts_rhs = rhs.into_le_parts();
		let parts_rhs = &parts_rhs as *const P;

		let mut result = MaybeUninit::<[P; N]>::uninit();
		let result_ptr = result.as_mut_ptr() as *mut P;

		let mut carry = false;

		unsafe {
			for i in 0..N {
				let s = *parts_self.add(i);
				let r = *parts_rhs.add(i);

				let (res, overflow_1) = s.overflowing_add(r);
				let (res, overflow_2) = res.overflowing_add(P::from_bool(carry));

				result_ptr.add(i).write(res);
				carry = overflow_1 || overflow_2;
			}
		}

		debug_assert!(!carry, "addition overflowed");
		Self::from_le_parts(unsafe { result.assume_init() })
	}

	// fn add(self, rhs: Self) -> Self {
	// 	let parts_self = self.into_le_parts();
	// 	let parts_rhs = rhs.into_le_parts();

	// 	let parts_self = &parts_self as *const P;
	// 	let parts_rhs = &parts_rhs as *const P;

	// 	let mut result = MaybeUninit::<[P; N]>::uninit();
	// 	let result_ptr = result.as_mut_ptr() as *mut P;

	// 	unsafe {
	// 		for i in 0..N {
	// 			let s = *parts_self.add(i);
	// 			let r = *parts_rhs.add(i);

	// 			// let part = s
	// 			result_ptr.add(i).write(part);
	// 		}

	// 		Self::from_le_parts(result.assume_init())
	// 	}
	// }
}

// TODO: https://doc.rust-lang.org/nightly/std/primitive.u128.html
pub trait Part
where
	Self: Sized,
	Self: Copy
	// Self: Add,
	// Self: Sub,
	// Self: Mul,
	// Self: Div,
	// Self: Neg,
	// Self: Rem,
	// Self: AddAssign,
	// Self: SubAssign,
	// Self: MulAssign,
	// Self: DivAssign,
	// Self: RemAssign,
	// Self: BitAnd,
	// Self: BitOr,
	// Self: BitXor,
	// Self: Not,
	// Self: Shl,
	// Self: Shr,
	// Self: BitAndAssign,
	// Self: BitOrAssign,
	// Self: BitXorAssign,
	// Self: ShlAssign,
	// Self: ShrAssign
{
	fn from_bool(b: bool) -> Self;
	fn overflowing_add(self, rhs: Self) -> (Self, bool);
}

macro_rules! uint_impl_part {
	($($ty:ty)*) => {
		$(
			impl Part for $ty {
				fn from_bool(b: bool) -> Self { b as _ }
				fn overflowing_add(self, rhs: Self) -> (Self, bool) { self.overflowing_add(rhs) }
			}
		)*
	}
}

uint_impl_part! { u8 u16 u32 u64 u128 }

#[derive(Clone, Debug)]
pub struct U320 {
	pub inner: [u64; 5]
}

impl UnsignedBigint<u64, 5> for U320 {
	const MIN: Self = Self { inner: [u64::MIN; 5] };
	const MAX: Self = Self { inner: [u64::MAX; 5] };
	const BITS: u32 = 320;

	fn into_le_parts(self) -> [u64; 5] {
		self.inner
	}

	fn from_le_parts(parts: [u64; 5]) -> Self {
		Self { inner: parts }
	}
}

/*
I think we always would want to use unsigned for container type? so we can manage sign bit ourself? I think?

ideally:
- ints from 1bit to 128 bit using one of the smallest container type (u8, u16, etc)
- ints from 1 bit to <arbitrary> using either just u8's or conbination of smallest evenly divisible type or something
	- I guess "packed representation"?
	- 1 to 8 use u8
	- 9 to 16 use u16 (no benefit in 2xu8)
	- 17 to 24 use 3xu8
	- 25 to 32 use u32
	- 33 to 40 use 5xu8
	- 41 to 48 use 3xu16
	- 49 to 56 use 7xu8
	- 57 to 64 use u64
	- etc
	- we go above 128-bit too, bigints, but probably have to have it gated behind features
- and for the ones that can be evenly put into even amount of larger-than-u8 types (eg. u48, 3xu16), offer structs with its smaller align form (eg. 6xu8)
- combo types..? like, u7u9 or so, that can pack the two into only two bytes etc
*/
