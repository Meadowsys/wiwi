use crate::num_traits::*;
use std::mem::MaybeUninit;

// TODO: generic params can't be used in const exprs because I dunno why
// so we just return 2 arrays, in le order (so it can be transmuted to [I; BYTES * 2])
pub fn mul_widening<const BYTES: usize, I>(
	int1: [I; BYTES],
	int2: [I; BYTES]
) -> [[I; BYTES]; 2]
where
	I: AddCarrying + AddOverflowing + MulWidening + Copy
{
	// SAFETY: it is not possible to overflow `result`:
	// - `result` is double the size of one input array of length `BYTES`
	// - the two loops will iterate to max `BYTES - 1` each
	// - `i_outer + i_inner` will be no larger than `(2 * BYTES) - 2`, which is
	//   less than `2 * BYTES`
	// - we only loop until (BYTES * 2), which will stay in bounds of the array
	// - squaring a number `n` is the same as 2.pow(2 * log2(n)), which is saying,
	//   for any number, it's bit width will not more than double when squaring it
	// so, [[I; BYTES]; 2] is enough length, and it's not possible to overflow it,
	// both in the code and in the arithmetic result
	unsafe {
		let int1_ptr = int1.as_ptr();
		let int2_ptr = int2.as_ptr();

		let mut result = [[I::ZERO; BYTES]; 2];
		let result_ptr = result.as_mut_ptr().cast::<I>();

		for i_outer in 0..BYTES {
			let i1 = *int1_ptr.add(i_outer);
			for i_inner in 0..BYTES {
				let i2 = *int2_ptr.add(i_inner);

				let (l, h) = I::mul_widening(i1, i2);

				let base = i_outer + i_inner;
				let mut base_ptr = result_ptr.add(base);

				let (res, carry) = (*base_ptr).add_overflowing(l);
				base_ptr.write(res);
				base_ptr = base_ptr.add(1);

				let (mut res, mut carry) = (*base_ptr).add_carrying(h, carry);
				base_ptr.write(res);
				base_ptr = base_ptr.add(1);

				for _ in (base + 2)..(BYTES * 2) {
					if !carry { break }

					let (r, c) = (*base_ptr).add_overflowing(I::ONE);
					base_ptr.write(r);

					base_ptr = base_ptr.add(1);
					carry = c;
				}

				debug_assert!(!carry, "invalid state (we cannot overflow)");
			}
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::{ RngCore, Rng, thread_rng };
	use std::mem::transmute;

	#[test]
	fn overflowing_random_u32_u64() {
		for _ in 0..1000 {
			let orig_int1 = thread_rng().next_u32();
			let orig_int2 = thread_rng().next_u32();
			let expected = orig_int1.into_u64() * orig_int2.into_u64();

			let int1 = orig_int1.to_le_bytes();
			let int2 = orig_int2.to_le_bytes();

			let res = mul_widening(int1, int2);
			let res = u64::from_le_bytes(unsafe { transmute(res) });

			assert_eq!(expected, res);
		}
	}
}
