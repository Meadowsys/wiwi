//! Fast and efficient implementation of Z85, but with nonstandard padding
//! implemented, as the Z85 spec does not account for padding.
//!
//! ## Nonstandard padding implementation
//!
//! It may be worth noting that no extra bytes are added if the given input is
//! to the correct length (multiple of 4).
//!
//! **Encoding**: If padding is needed, the amount of padding that was added in bytes
//! is encoded (ex. 1B padding -> `1` since `TABLE_ENCODER[1] == b'1'`)
//! and appended to the end of the string. (1 extra byte)
//!
//! **Decoding**: If the len of the slice passed is one more than a multiple of 5
//! (ie. `(n * 5) + 1`), it is trimmed off the slice, decoded to get amount of
//! padding needed, and held onto. Then while decoding the last frame, we take
//! that stored amount of padding, and remove that amount from the end of the
//! decoded bytes.
//!
//! Original Z85 spec: <https://rfc.zeromq.org/spec/32>

use crate::encoding_utils::{ ChunkedSlice, UnsafeBufWriteGuard};
use std::slice;

pub const TABLE_ENCODER_LEN: usize = 85;
pub const TABLE_ENCODER: [u8; TABLE_ENCODER_LEN] = *b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.-:+=^!/*?&<>()[]{}@%$#";
//                                                    0123456789          0123456789          0123456789          0123456789          01234
//                                                              0123456789          0123456789          0123456789          0123456789
//                                                    0         10        20        30        40        50        60        70        80

pub const TABLE_DECODER_LEN: usize = 256;
// generated by `generate-z85-table-decoder`, a binary in this crate
pub const TABLE_DECODER: [Option<u8>; TABLE_DECODER_LEN] = [
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       Some(0x44), None,       Some(0x54), Some(0x53), Some(0x52), Some(0x48), None,       Some(0x4b), Some(0x4c), Some(0x46), Some(0x41), None,       Some(0x3f), Some(0x3e), Some(0x45),
	Some(0x00), Some(0x01), Some(0x02), Some(0x03), Some(0x04), Some(0x05), Some(0x06), Some(0x07), Some(0x08), Some(0x09), Some(0x40), None,       Some(0x49), Some(0x42), Some(0x4a), Some(0x47),
	Some(0x51), Some(0x24), Some(0x25), Some(0x26), Some(0x27), Some(0x28), Some(0x29), Some(0x2a), Some(0x2b), Some(0x2c), Some(0x2d), Some(0x2e), Some(0x2f), Some(0x30), Some(0x31), Some(0x32),
	Some(0x33), Some(0x34), Some(0x35), Some(0x36), Some(0x37), Some(0x38), Some(0x39), Some(0x3a), Some(0x3b), Some(0x3c), Some(0x3d), Some(0x4d), None,       Some(0x4e), Some(0x43), None,
	None,       Some(0x0a), Some(0x0b), Some(0x0c), Some(0x0d), Some(0x0e), Some(0x0f), Some(0x10), Some(0x11), Some(0x12), Some(0x13), Some(0x14), Some(0x15), Some(0x16), Some(0x17), Some(0x18),
	Some(0x19), Some(0x1a), Some(0x1b), Some(0x1c), Some(0x1d), Some(0x1e), Some(0x1f), Some(0x20), Some(0x21), Some(0x22), Some(0x23), Some(0x4f), None,       Some(0x50), None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None
];

pub const BINARY_FRAME_LEN: usize = 4;
pub const STRING_FRAME_LEN: usize = 5;

/// Encodes a slice of bytes into a Z85 string, adding padding if necessary
pub fn encode_z85(bytes: &[u8]) -> String {
	// we *don't* fast path out on zero bytes, because in like, 99% of situations,
	// the input is not 0 length, lol. if it were, frames and remainder would be 0,
	// capacity would end up 0 (so no allocation is made), the first loop wouldn't
	// run, the remainder if block wouldn't run, debug_assert's pass, and String is
	// created from empty Vec (empty string, no allocation too). so all is good
	// functionality wise, and its still a fairly fast exit too, I think.

	// right shift 2 is same as integer divide by 4 (BINARY_FRAME_LEN)
	let frames = bytes.len() >> 2;

	// binary AND with 0b11 (3) is the same as modulo 4 (BINARY_FRAME_LEN)
	let remainder = bytes.len() & 0b11;

	// preallocate exact amount of memory needed
	let capacity = if remainder == 0 {
		frames * STRING_FRAME_LEN
	} else {
		// frames is number of *whole* binary frames, so the remainder is not
		// included in this. adding 1 to allocate space for a whole frame for it.
		let capacity = (frames + 1) * STRING_FRAME_LEN;
		// don't forget that last byte that encodes amount of padding
		capacity + 1
	};

	let mut frames_iter = ChunkedSlice::<BINARY_FRAME_LEN>::new(bytes);
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);

	for _ in 0..frames {
		// SAFETY: the loop will loop `frames` times, each time taking out a frame's worth
		// of bytes (`next_frame_unchecked`). this will consume all the bytes except
		// amount stored in remainder. We also preallocated enough memory up front,
		// so passing in &mut dest is fine.
		unsafe {
			let frame = frames_iter.next_frame_unchecked();
			encode_frame(frame, &mut dest);
		}
	}

	if remainder > 0 {
		unsafe {
			// SAFETY: this will only run if there is any remainder, which is correct.
			// `remainder` contains a frame with static len 4 (as required by
			// encode_frame too). We preallocated enough memory up front, so &mut dest
			// is good to use.
			frames_iter.with_remainder_unchecked(|remainder| {
				encode_frame(remainder, &mut dest);
			});

			// amount of bytes that are padding (ie. doens't contain data)
			let padding_len = BINARY_FRAME_LEN - remainder;

			// SAFETY: 0 < padding_len < 4 will always be true, because of calculation
			// of remainder (bytes.len() % 4 will always be 0 <= n <= 3) which
			// fits in TABLE_ENCODER_LEN (which is 85).
			let padding_char = *TABLE_ENCODER.get_unchecked(padding_len);

			// SAFETY: we would have preallocated this one extra byte if remainder
			// were needed.
			dest.write_bytes_const::<1>(&padding_char);
		}
	}

	// SAFETY: we preallocated an amount of memory up front, and written enough
	// bytes already. this will always be safe
	let vec = unsafe { dest.into_full_vec() };

	// SAFETY: we only are pushing in chars in the table, which are all valid ASCII.
	debug_assert!(String::from_utf8(vec.clone()).is_ok(), "output bytes are valid utf-8");
	unsafe { String::from_utf8_unchecked(vec) }
}

/// Decodes a slice of of a Z85 string back into the source bytes
pub fn decode_z85(mut bytes: &[u8]) -> Result<Vec<u8>, DecodeError> {
	if bytes.len() < STRING_FRAME_LEN {
		return if bytes.is_empty() {
			Ok(Vec::new())
		} else {
			// in here, bytes len is 0 < n < STRING_FRAME_LEN. we already returned
			// on empty input (valid because empty bytes <-> empty string). at input
			// lengths 1-3, the single frame would have been padded to a full frame
			// and then the amount of padding appended as one more byte, for a total
			// lenght of 6. At input length 4, it would just be the frame without
			// any extra bytes added. so therefore the smallest valid non-zero len
			// is 5, encoding one full frame of data or more, so this is invalid input.
			Err(DecodeError::InvalidLength)
		}
	}

	// `bytes.len()` will always be 5 or more, so `frames` will always
	// be 1 or more (see comment above)
	let frames = bytes.len() / STRING_FRAME_LEN;
	debug_assert!(frames >= 1, "condition of \"at least one frame in input\" was checked correctly");

	let remainder = bytes.len() % STRING_FRAME_LEN;

	// left shift 2 is the same as multiply by 4 (BINARY_FRAME_LEN)
	let capacity = frames << 2;

	// Match statement to check remainder for that extra padding encoding byte.
	let (capacity, added_padding) = match remainder {
		0 => {
			// no padding was added
			(capacity, 0usize)
		}
		1 => {
			// the singular padding byte (there will never be more than this, handled
			// by below match case, returning error)
			let added_padding = unsafe {
				// SAFETY: remainder is 1, so there will be at least 1 byte in the
				// slice, because duh (well technically there will be at least 6
				// bytes but at least 1 still holds). so this will not underflow
				let one_shorter = bytes.len() - 1;

				let ptr = bytes as *const [u8] as *const u8;
				// this will be last byte in slice
				let byte = *(ptr.add(one_shorter));

				// SAFETY: `one_shorter` is bytes.len() - 1, which as explained
				// above is safe, so this will be too.
				bytes = slice::from_raw_parts(ptr, one_shorter);

				// SAFETY: 0 <= n < 256 is always true for a u8, and TABLE_DECODER is len 256,
				// so this is safe
				let decoded = *TABLE_DECODER.get_unchecked(byte as usize);

				match decoded {
					// having that last byte as a 0 is not something we generate,
					// as its just a waste of a perfectly good byte, but it doesn't
					// break this system (added a unit test for it to make sure).
					Some(val) if (val as usize) < BINARY_FRAME_LEN => { val }
					Some(_) | None => { return Err(DecodeError::InvalidChar) }
				}
			} as usize;

			// if added_padding is 0, this returns
			// the same values as the above.
			(capacity - added_padding, added_padding)
		}
		_n => { return Err(DecodeError::InvalidLength) }
	};

	// because frames >= 1, `excluding_last_frame` will be >= 0 (ie. will not underflow).
	let excluding_last_frame = frames - 1;

	let mut frames_iter = ChunkedSlice::<STRING_FRAME_LEN>::new(bytes);
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);

	for _ in 0..excluding_last_frame {
		unsafe {
			// SAFETY: this loop loops `excluding_last_frame` times, ie. loops through
			// every frame except the last. We've also preallocated enough capacity
			// for the bytes we will write
			let frame = frames_iter.next_frame_unchecked();
			decode_frame(frame, |frame| dest.write_bytes_const::<BINARY_FRAME_LEN>(frame as *const u8))?;
		}
	}

	// this is the last frame, and this is where the padding is handled
	unsafe {
		let frame = frames_iter.next_frame_unchecked();
		decode_frame(frame, |frame| {
			// - if 0 bytes of padding were added, this is whole frame and
			//   added_padding would be 0
			// - if 0 < n < 4 bytes of padding were added, this is correct
			// - if 4 <= n bytes of "padding" were added, this should have been
			//   either be 0 or 0 < n < 4, ie. this case would not happen.
			// this is checked up at the top, where the padding amount is decoded

			// This is the amount of bytes minus the padding bytes at the end.
			// Because of all that explained above, this will also be in range of
			// 0 <= n < BINARY_FRAME_LEN, ie. will not underflow. And because
			// we are subtracting from BINARY_FRAME_LEN with a number 0 or more up to
			// one before BINARY_FRAME_LEN, this will be 1 <= n <= BINARY_FRAME_LEN.
			let non_padding_bytes = BINARY_FRAME_LEN - added_padding;

			// SAFETY: this writes the amount of bytes that aren't padding bytes,
			// into the buffer. We subtracted padding bytes from the number we write
			// already, so we write the perfect amount left and never over or under.
			dest.write_bytes(frame as *const u8, non_padding_bytes);
		})?;
	}

	// SAFETY: We have consumed all the input bytes (calculated)
	#[cfg(debug_assertions)]
	frames_iter.debug_assert_is_empty();

	// SAFETY: We have written the exact amount of bytes we preallocated (calculated)
	Ok(unsafe { dest.into_full_vec() })
}

// TODO: these errors could be improved.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DecodeError {
	#[error("invalid length")]
	InvalidLength,
	#[error("invalid character")]
	InvalidChar,
	#[error("correct characters, but incorrect combination that would cause overflow")]
	InvalidFrame
}

/// # Safety
///
/// Caller must guarantee dest is valid for at least `STRING_FRAME_LEN` bytes
/// to be written.
unsafe fn encode_frame(frame: &[u8; BINARY_FRAME_LEN], dest: &mut UnsafeBufWriteGuard) {
	let mut int = u32::from_be_bytes(*frame) as usize;

	let byte5 = int % TABLE_ENCODER_LEN;
	int /= TABLE_ENCODER_LEN;

	let byte4 = int % TABLE_ENCODER_LEN;
	int /= TABLE_ENCODER_LEN;

	let byte3 = int % TABLE_ENCODER_LEN;
	int /= TABLE_ENCODER_LEN;

	let byte2 = int % TABLE_ENCODER_LEN;
	int /= TABLE_ENCODER_LEN;

	let byte1 = int;

	debug_assert!(int % TABLE_ENCODER_LEN == int, "no remaining/unused byte information");
	debug_assert!(int / TABLE_ENCODER_LEN == 0, "no remaining/unused byte information");

	// SAFETY: these are calculated by modulo TABLE_ENCODER_LEN (85), which
	// guarantees the numbers are 0 <= n < TABLE_ENCODER_LEN (85),
	// and the length of TABLE_ENCODER is 85, so this won't overflow
	let encoded_frame: [u8; STRING_FRAME_LEN] = unsafe { [
		*TABLE_ENCODER.get_unchecked(byte1),
		*TABLE_ENCODER.get_unchecked(byte2),
		*TABLE_ENCODER.get_unchecked(byte3),
		*TABLE_ENCODER.get_unchecked(byte4),
		*TABLE_ENCODER.get_unchecked(byte5),
	] };

	// SAFETY: internal function. caller guarantees past dest has at least
	// `STRING_FRAME_LEN` bytes left.
	dest.write_bytes_const::<STRING_FRAME_LEN>(&encoded_frame as *const u8);
}

/// # Safety
///
/// All allowed-by-type-system inputs are valid. However, marking this function
/// `unsafe` is not only consistent with [`encode_frame`], its also just more
/// convenient :p. This is an internal function, so doesn't matter too much.
unsafe fn decode_frame<F>(frame: &[u8; STRING_FRAME_LEN], f: F) -> Result<(), DecodeError>
where
	F: FnOnce(&[u8; BINARY_FRAME_LEN])
{
	let [byte1, byte2, byte3, byte4, byte5] = *frame;

	// SAFETY: 0 <= n < 256 is always true for a u8, and TABLE_DECODER is len 256,
	// so this is safe.
	// Additionally, if this comes back as Some from TABLE_DECODER, it is guaranteed
	// to be 0 <= n <= 84, since there are no Some(n) values outside this range.
	let Some(byte1) = *TABLE_DECODER.get_unchecked(byte1 as usize) else {
		return Err(DecodeError::InvalidChar)
	};
	let Some(byte2) = *TABLE_DECODER.get_unchecked(byte2 as usize) else {
		return Err(DecodeError::InvalidChar)
	};
	let Some(byte3) = *TABLE_DECODER.get_unchecked(byte3 as usize) else {
		return Err(DecodeError::InvalidChar)
	};
	let Some(byte4) = *TABLE_DECODER.get_unchecked(byte4 as usize) else {
		return Err(DecodeError::InvalidChar)
	};
	let Some(byte5) = *TABLE_DECODER.get_unchecked(byte5 as usize) else {
		return Err(DecodeError::InvalidChar)
	};

	// reversal process of encode. Max value is 84^5, or 4.182.119.424, which
	// is less than u32::MAX, or 4.294.967.296, so will not overflow.
	let mut int = byte1 as u64;

	int *= TABLE_ENCODER_LEN as u64;
	int += byte2 as u64;

	int *= TABLE_ENCODER_LEN as u64;
	int += byte3 as u64;

	int *= TABLE_ENCODER_LEN as u64;
	int += byte4 as u64;

	int *= TABLE_ENCODER_LEN as u64;
	int += byte5 as u64;

	if int >> u32::BITS != 0 { return Err(DecodeError::InvalidFrame) }

	let decoded_frame = u32::to_be_bytes(int as u32);
	f(&decoded_frame);

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::{ Rng, thread_rng };

	#[test]
	fn provided_test_case() {
		let bytes: &[u8] = &[
			0x86, 0x4f, 0xd2, 0x6f,
			0xb5, 0x59, 0xf7, 0x5b
		];
		let encoded = "HelloWorld";

		assert_eq!(encoded, encode_z85(bytes));
		assert_eq!(bytes, decode_z85(encoded.as_bytes()).expect("provided test case decodes properly"));
	}

	#[test]
	fn randomised() {
		// (bytes_len, encoded_len)
		// (expected_input_len, expected_output_len)
		let expected_lengths = [
			(0usize, 0usize),
			(1, 6),
			(2, 6),
			(3, 6),
			(4, 5),
			(5, 11),
			(6, 11),
			(7, 11),
			(8, 10),
			(9, 16),
			(10, 16),
			(11, 16),
			(12, 15),
			(13, 21),
			(14, 21),
			(15, 21),
			(16, 20),
			(17, 26),
			(18, 26),
			(19, 26),
			(20, 25),

			(50, 66),
			(100, 125),
			(500, 625),
			(1000, 1250),
			(100_000, 125_000),
			(1_000_000, 1_250_000),
		];
		let mut rng = thread_rng();

		for (expected_input_len, expected_output_len) in expected_lengths {
			for _ in 0usize..5 {
				let mut original_input = vec![0u8; expected_input_len];
				rng.fill(&mut *original_input);
				assert_eq!(original_input.len(), expected_input_len);

				let encoded = encode_z85(&original_input);
				assert_eq!(encoded.len(), expected_output_len);

				let decoded = decode_z85(encoded.as_bytes())
					.expect("can round trip decode just encoded data");
				assert_eq!(decoded.len(), expected_input_len);

				assert_eq!(original_input, decoded);

				// this is enforced by debug_assert! in the code, so this already
				// is validated if tests are run in debug, but still,
				assert_eq!(encoded.len(), encoded.capacity());
				assert_eq!(decoded.len(), decoded.capacity());
			}
		}
	}

	#[test]
	fn z85_crate_nonpadded_compat() {
		// this should work, since when bytes length is a multiple of 4
		// we don't add any padding characters and `z85` doesn't add any
		// either, meaning in this situation our impls are cross compatible

		let mut rng = thread_rng();

		let mut bytes = vec![0u8; 1000];
		rng.fill(&mut *bytes);
		let bytes = &*bytes;

		let wiwi_encoded = encode_z85(bytes);
		let z85_encoded = z85::encode(bytes);
		assert_eq!(wiwi_encoded, z85_encoded);

		let wiwi_decoded_z85 = decode_z85(z85_encoded.as_bytes())
			.expect("wiwi can decode z85");
		let z85_decoded_wiwi = z85::decode(wiwi_encoded.as_bytes())
			.expect("z85 can decode wiwi");

		assert_eq!(wiwi_decoded_z85, z85_decoded_wiwi);
	}

	#[test]
	fn extra_zero_padding_byte() {
		// for the case where theres an extra padding marker byte that encodes
		// zero padding, which we don't emit because that's just a waste of space.
		// but it doesn't break the parser, so we don't check for it.

		let strs = [
			("adfeg", "adfeg0"),
			(
				// len 45
				"abcdefafuehirugehdbfntkvdneoiwr4htrugitdfkwwu",
				"abcdefafuehirugehdbfntkvdneoiwr4htrugitdfkwwu0"
			)
		];

		for (str1, str2) in strs {
			let str1 = decode_z85(str1.as_bytes())
				.expect("nonpadded z85 parses successfully");
			let str2 = decode_z85(str2.as_bytes())
				.expect("padded-with-0 z85 parses successfully");
			assert_eq!(str1, str2);
		}
	}

	#[test]
	fn max_value() {
		let decoded = decode_z85(b"%nSc0%nSc0%nSc0%nSc0");
		let decoded = decoded.as_deref();
		assert_eq!(decoded, Ok(&[255u8; 16] as &[u8]));
	}

	#[test]
	fn rejects_too_large() {
		let strs = [
			//   v
			"%nSc1",
			//                            v
			"%nSc0%nSc0%nSc0%nSc0%nSc0%nSc1%nSc0",
			//          v
			"%nSc0%nSc0%oSc0%nSc0%nSc0%nSc0%nSc0",
			// kinda obvious
			"#####"
		];

		for s in strs {
			let decoded = decode_z85(s.as_bytes());
			assert_eq!(decoded, Err(DecodeError::InvalidFrame))
		}
	}
}
