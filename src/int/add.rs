use crate::num_traits::Int;
use std::mem::MaybeUninit;

// /// Performs standard addition, with overflow checking depending on overflow
// /// checks compiler option
// ///
// /// NOTE: `cfg(overflow_checks)` is for some reason still unstable, so for now,
// /// without the `nightly` feature, this will panic on overflow depending on
// /// `cfg(debug_assertions)` (which is usually enabled in debug and disabled in release)
// pub fn add<I: Int<BYTES_PER_INT>, const BYTES_PER_INT: usize, const BYTES: usize>(
// 	int1: [I; BYTES],
// 	int2: [I; BYTES]
// ) -> [I; BYTES] {
// 	let (res, overflow) = overflowing_add(int1, int2);
//
// 	#[cfg_attr(not(feature = "nightly"), cfg(debug_assertions))]
// 	#[cfg_attr(feature = "nightly", cfg(overflow_checks))]
// 	assert!(!overflow, "attempt to add with overflow");
//
// 	res
// }

// pub fn checked_add<I: Int<BYTES_PER_INT>, const BYTES_PER_INT: usize, const BYTES: usize>(
// 	int1: [I; BYTES],
// 	int2: [I; BYTES]
// ) -> Option<[I; BYTES]> {
// 	let (res, overflow) = overflowing_add(int1, int2);
// 	if overflow { None } else { Some(res) }
// }

// pub fn strict_add<I: Int<BYTES_PER_INT>, const BYTES_PER_INT: usize, const BYTES: usize>(
// 	int1: [I; BYTES],
// 	int2: [I; BYTES]
// ) -> [I; BYTES] {
// 	let (res, overflow) = overflowing_add(int1, int2);
// 	assert!(!overflow, "attempt to add with overflow");
// 	res
// }

// pub unsafe fn unchecked_add<I: Int<BYTES_PER_INT>, const BYTES_PER_INT: usize, const BYTES: usize>(
// 	int1: [I; BYTES],
// 	int2: [I; BYTES]
// ) -> [I; BYTES] {
// 	let (res, overflow) = overflowing_add(int1, int2);
// 	res
// }

// pub fn saturating_add<I: Int<BYTES_PER_INT>, const BYTES_PER_INT: usize, const BYTES: usize>(
// 	int1: [I; BYTES],
// 	int2: [I; BYTES]
// ) -> [I; BYTES] {
// 	let (res, overflow) = overflowing_add(int1, int2);
// 	if overflow { [I::MAX; BYTES] } else { res }
// }

// pub fn wrapping_add<I: Int<BYTES_PER_INT>, const BYTES_PER_INT: usize, const BYTES: usize>(
// 	int1: [I; BYTES],
// 	int2: [I; BYTES]
// ) -> [I; BYTES] {
// 	let (res, overflow) = overflowing_add(int1, int2);
// 	res
// }

pub fn overflowing_add<I: Int<BYTES_PER_INT>, const BYTES_PER_INT: usize, const BYTES: usize>(
	int1: [I; BYTES],
	int2: [I; BYTES]
) -> ([I; BYTES], bool) {
	unsafe {
		let int1_ptr = int1.as_ptr();
		let int2_ptr = int2.as_ptr();

		let mut result = MaybeUninit::<[I; BYTES]>::uninit();
		let result_ptr = result.as_mut_ptr() as *mut I;

		let mut carry = false;

		for i in 0..BYTES {
			let i1 = *int1_ptr.add(i);
			let i2 = *int2_ptr.add(i);

			let (res, carry1) = i1.overflowing_add(i2);
			let (res, carry2) = res.overflowing_add(I::from_bool(carry));

			result_ptr.add(i).write(res);
			carry = carry1 || carry2;
		}

		(result.assume_init(), carry)
	}
}
