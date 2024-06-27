use crate::num_traits::WideningUnsignedInt;
use std::mem::MaybeUninit;

// TODO: generic params can't be used in const exprs because I dunno why
// so we just return 2 arrays, in le order (so it can be transmuted to [I; BYTES * 2])
pub fn overflowing_mul<
	const BYTES_PER_INT: usize,
	const BYTES: usize,
	const BYTES_PER_INT_WIDENED: usize,
	I: WideningUnsignedInt<BYTES_PER_INT, BYTES_PER_INT_WIDENED>
>(
	int1: [I; BYTES],
	int2: [I; BYTES]
) -> [[I; BYTES]; 2] {
	// it is not possible to overflow the double sized array

	unsafe {
		let int1_ptr = int1.as_ptr();
		let int2_ptr = int2.as_ptr();

		let mut result = [[I::ZERO; BYTES]; 2];
		let result_ptr = result.as_mut_ptr() as *mut I;

		for i_outer in 0..BYTES {
			let i1 = *int1_ptr.add(i_outer);
			for i_inner in 0..BYTES {
				let i2 = *int2_ptr.add(i_inner);

				let wide = I::mul_widening_nosplit(i1, i2);
				let (l, h) = I::split_wide(wide);

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
	use std::mem::transmute;

	#[test]
	fn basic() {
		let orig_int1 = 238785764u32;
		let orig_int2 = 2256886u32;
		let expected = orig_int1 as u64 * orig_int2 as u64;

		let int1 = orig_int1.to_le_bytes();
		let int2 = orig_int2.to_le_bytes();

		let res = overflowing_mul(int1, int2);
		let res = u64::from_le_bytes(unsafe { transmute(res) });

		assert_eq!(expected, res);
	}
}
