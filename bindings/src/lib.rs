#![allow(
	clippy::missing_safety_doc
)]

use std::{ ptr, slice };
use std::mem::{ ManuallyDrop, MaybeUninit };
use wiwi::z85::{ decode_z85, encode_z85 };

#[repr(C)]
pub struct Z85Input {
	ptr: *const u8,
	len: usize
}

#[repr(C)]
pub struct Z85Result {
	ptr: *const u8,
	len: usize,
	cap: usize
}

#[no_mangle]
pub unsafe extern "C" fn wiwi_z85_encode(input: &Z85Input, output: &mut MaybeUninit<Z85Result>) {
	let bytes = slice::from_raw_parts(input.ptr, input.len);
	let res = ManuallyDrop::new(encode_z85(bytes).into_bytes());

	let res = Z85Result {
		ptr: res.as_ptr(),
		len: res.len(),
		cap: res.capacity()
	};

	output.write(res);
}

pub unsafe extern "C" fn wiwi_z85_decode(input: &Z85Input, output: &mut MaybeUninit<Z85Result>) {
	let bytes = slice::from_raw_parts(input.ptr, input.len);

	let res = match decode_z85(bytes) {
		Ok(vec) => {
			let slice = ManuallyDrop::new(vec);

			Z85Result {
				ptr: slice.as_ptr(),
				len: slice.len(),
				cap: slice.capacity()
			}
		}
		Err(_) => {
			Z85Result {
				ptr: ptr::null(),
				len: 0,
				cap: 0
			}
		}
	};

	output.write(res);
}

#[no_mangle]
pub unsafe extern "C" fn wiwi_drop_z85_result(res: &Z85Result) {
	// let res = &*res;
	if !res.ptr.is_null() {
		drop(unsafe { Vec::from_raw_parts(res.ptr.cast_mut(), res.len, res.len) })
	}
}
