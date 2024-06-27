use crate::num_traits::UnsignedInt;
use std::mem::MaybeUninit;

fn overflowing_sub<I: UnsignedInt<BYTES_PER_INT>, const BYTES_PER_INT: usize, const BYTES: usize>(
	int1: [I; BYTES],
	int2: [I; BYTES]
) -> ([I; BYTES], bool) {
	unsafe {
		let int1_ptr = int1.as_ptr();
		let int2_ptr = int2.as_ptr();

		let mut result = MaybeUninit::<[I; BYTES]>::uninit();
		let result_ptr = result.as_mut_ptr() as *mut I;

		let mut borrow = false;

		for i in 0..BYTES {
			let i1 = *int1_ptr.add(i);
			let i2 = *int2_ptr.add(i);

			let (res, borrow1) = i1.sub_overflowing(i2);
			let (res, borrow2) = res.sub_overflowing(I::from_bool(borrow));

			result_ptr.add(i).write(res);
			borrow = borrow1 || borrow2;
		}

		(result.assume_init(), borrow)
	}
}
