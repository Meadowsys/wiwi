use super::{ *, buffer::*, error::*, serialise::*, deserialise::* };
use std::{ ptr, slice };

exported_match_macro!(smallint_positive_range: 0x00..=0x7f);
exported_match_macro!(smallint_negative_range: -64..=-1);
exported_match_macro!(smallint_negative_as_unsigned_range: 0xc0..=0xff);
exported_match_macro!(smallint_unsigned_range: 0x00..=0x7f);
exported_match_macro!(smallint_signed_range: -64..=0x7f);
exported_match_macro!(int_range: 0x80..=0x9f);

// macro_rules! impl_num_serialise {
// 	($($num:ty: $signed:tt)*) => {
// 		$(impl_num_serialise! { @internal $num: $signed })*
// 	};

// 	// unsigned
// 	(@internal $num:ty: false) => {
// 		impl Serialise for $num {
// 			fn serialise<B: BufferWrite>(&self, output: &mut B, options: &SerialiseOptions) {
// 				match *self {
// 					num @ smallint_positive_range!() => { output.write_byte(num as _) }
// 					num => {
// 						let bytes = num.to_le_bytes();

// 						let byte_size = get_byte_count_unsigned_le(bytes);
// 						let marker = unsafe { get_marker_for(byte_size, false) };

// 						output.write_byte(marker);
// 						output.write_bytes(unsafe { slice::from_raw_parts(
// 							&bytes as *const u8,
// 							byte_size as _
// 						) });
// 					}
// 				}
// 			}
// 		}
// 	};

// 	// signed
// 	(@internal $num:ty: true) => {
// 		impl Serialise for $num {
// 			fn serialise<B: BufferWrite>(&self, output: &mut B, options: &SerialiseOptions) {
// 				match *self {
// 					num @ smallint_positive_range!() => { output.write_byte(num as _) }
// 					num @ smallint_negative_range!() => { output.write_byte(num as _) }
// 					num => {
// 						let bytes = num.to_le_bytes();

// 						let byte_size = get_byte_count_signed_le(bytes);
// 						let marker = unsafe { get_marker_for(byte_size, true) };

// 						output.write_byte(marker);
// 						output.write_bytes(unsafe { slice::from_raw_parts(
// 							&bytes as *const u8,
// 							byte_size as _
// 						) });
// 					}
// 				}
// 			}
// 		}
// 	};
// }

// impl_num_serialise! {
// 	u8: false
// 	u16: false
// 	u32: false
// 	u64: false
// 	u128: false

// 	i8: true
// 	i16: true
// 	i32: true
// 	i64: true
// 	i128: true
// }

fn get_byte_count_unsigned_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	unsafe {
		let ptr = &bytes as *const u8;

		for i in (1..BYTES).rev() {
			// simple! just return the first byte (including the byte) where
			// its not all 0s. Iter stops after offset 1 because we'll always
			// return at least 1 byte, so no point in checking last byte.
			if *ptr.add(i) != 0 { return (i + 1) as _ }
		}

		1
	}
}

fn get_byte_count_signed_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	unsafe {
		let ptr = &bytes as *const u8;
		let sign_bit = *ptr.add(BYTES - 1) >> 7;
		// byte that has empty data and can (probably) be safely discarded.
		// if negative, all bits 1, if positive, all bits 0
		let empty_byte = if sign_bit == 0 { 0 } else { u8::MAX };

		for i in (0..BYTES).rev() {
			let byte = *ptr.add(i);

			// the following could be shortened to a one liner... but for clarity, nah
			// if byte != empty_byte { return (i + 1) as u8 + (byte >> 7 != sign_bit) as u8 }

			if byte == empty_byte {
				// byte is full of 1s if negative, or full of 0s if positive
				// this byte can (probably) be safely discarded; continue
			} else if byte >> 7 == sign_bit {
				// sign bit is the same, return up to / including this byte
				// iter range is 0 to BYTES - 1 (inclusive), so this return range
				// will be 1 to BYTES (inclusive), which is good
				return (i + 1) as _
			} else {
				// sign bit is different, return this byte and one more after it.
				// if the next byte would have the wrong sign, it would have returned
				// already in the previous branch. This won't ever overflow because
				// the first byte will not have a different sign (as... itself),
				// so will never reach here.
				return (i + 2) as _
			}
		}

		1
	}
}

/// # Safety
///
/// Function is written only expecting certain values for `byte_count` (`1..=16`),
/// and we have not thought about what happens if values are entered outside that
/// range. Also in how this function's output is going to be used, it can cause
/// memory issues if used in wrong way. So this fn is conservatively marked
/// `unsafe`, and in doing so, promise this function will behave as expected.
/// It's internal function anyways it doesn't matter that much :p
#[inline]
const unsafe fn get_marker_for(byte_count: u8, signed: bool) -> u8 {
	((byte_count - 1) << 1) | 0x80 | signed as u8
}

#[inline]
const fn is_int_marker(marker: u8) -> bool {
	marker >> 5 == 0b100
}

struct MarkerDetails {
	byte_size: u8,
	signed: bool
}

impl MarkerDetails {
	#[inline]
	pub fn try_new(marker: u8) -> Option<Self> {
		is_int_marker(marker).then(|| {
			let marker = marker & 0b11111;
			let byte_size = (marker >> 1) + 1;
			let signed = marker & 1 != 0;

			Self { byte_size, signed }
		})
	}
}

// // TODO: need to be able to handle floats too (deserialise proper if the float has no fractional part)

// // impl<'h> Deserialise<'h> for u8 {
// // 	fn deserialise<B: BufferRead<'h>>(input: &mut B) -> Result<Self> {
// // 		match MarkerDetails::try_new::<1>(input.read_byte()?) {
// // 			Some(MarkerDetails { byte_size, signed: false }) => {
// // 				let mut bytes = [0u8; 1];
// // 				// ptr::copy_nonoverlapping(
// // 				// 	// input.read_bytes_ptr(byte_size as _)
// // 				// )
// // 			}
// // 			_ => { return err_str("expected u8-compatible number") }
// // 		}
// //
// // 		todo!()
// // 	}
// // }
