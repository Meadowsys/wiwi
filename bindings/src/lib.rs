use std::{ ptr, slice };
use std::mem::{ ManuallyDrop, MaybeUninit };
use wiwi::z85::{ decode_z85, encode_z85 };

#[repr(C)]
struct Z85Input {
	ptr: *const u8,
	len: usize
}

#[repr(C)]
struct Z85Result {
	ptr: *const u8,
	len: usize,
	cap: usize
}

#[no_mangle]
extern "C" fn wiwi_z85_encode(input: &Z85Input, output: &mut MaybeUninit<Z85Result>) {
	let bytes = unsafe { slice::from_raw_parts(input.ptr, input.len) };
	let vec = ManuallyDrop::new(encode_z85(bytes).into_bytes());

	let res = Z85Result {
		ptr: vec.as_ptr(),
		len: vec.len(),
		cap: vec.capacity()
	};

	output.write(res);
}

#[no_mangle]
extern "C" fn wiwi_z85_decode(input: &Z85Input, output: &mut MaybeUninit<Z85Result>) {
	let bytes = unsafe { slice::from_raw_parts(input.ptr, input.len) };

	let res = match decode_z85(bytes) {
		Ok(vec) => {
			let vec = ManuallyDrop::new(vec);

			Z85Result {
				ptr: vec.as_ptr(),
				len: vec.len(),
				cap: vec.capacity()
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
extern "C" fn wiwi_z85_drop_result(res: &Z85Result) {
	if !res.ptr.is_null() {
		drop(unsafe { Vec::from_raw_parts(res.ptr.cast_mut(), res.len, res.cap) })
	}
}
