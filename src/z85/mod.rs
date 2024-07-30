use crate::_internal::encoding_utils::{ ChunkedSlice, UnsafeBufWriteGuard};
use crate::num_traits::*;
use std::{ slice, str };

/// Length of the encoding table (ie. number of different characters)
pub const TABLE_ENCODER_LEN: usize = 85;
/// Table to map a number 0-84 to a Z85 character.
pub static TABLE_ENCODER: [u8; TABLE_ENCODER_LEN] = *b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.-:+=^!/*?&<>()[]{}@%$#";
//                                                     0123456789          0123456789          0123456789          0123456789          01234
//                                                               0123456789          0123456789          0123456789          0123456789
//                                                     0         10        20        30        40        50        60        70        80

/// Length of the decoding table
pub const TABLE_DECODER_LEN: usize = 256;

/// Table of length 256, mapping any `u8` to the value it represents
///
/// `0xff` values represent `None`. Doing it this way instead of Option<u8> halves
/// the size of this table (u8 is 1 byte, Option<u8> is 2 bytes)
// generated by script `generate-z85-table-decoder`... and then manually modified
// when we switched over from Option<u8> to just u8 with 0xff representing None
pub static TABLE_DECODER: [u8; 256] = [
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,

	NONE, 0x44, NONE, 0x54, 0x53, 0x52, 0x48, NONE, 0x4b, 0x4c, 0x46, 0x41, NONE, 0x3f, 0x3e, 0x45,
	0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x40, NONE, 0x49, 0x42, 0x4a, 0x47,
	0x51, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32,
	0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x4d, NONE, 0x4e, 0x43, NONE,
	NONE, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
	0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x4f, NONE, 0x50, NONE, NONE,

	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE
];
const NONE: u8 = 0xff;

/// One frame of this many bytes (4) from the binary data encodes to
/// one frame of 5 ([`STRING_FRAME_LEN`]) bytes of encoded Z85 output
pub const BINARY_FRAME_LEN: usize = 4;

/// One frame of this many bytes (5) from the encoded Z85 data decodes
/// to one frame of 4 ([`BINARY_FRAME_LEN`]) bytes of binary data.
pub const STRING_FRAME_LEN: usize = 5;

/// Encodes a slice of bytes into a Z85 string, adding padding if necessary
pub fn encode_z85(bytes: &[u8]) -> String {
	// we *don't* fast path out on zero bytes, because in like, 99%
	// of situations, the input is not 0 length, lol

	let EncodedReprInfo {
		frames,
		remainder,
		needed_capacity
	} = EncodedReprInfo::for_input_len(bytes.len());

	let mut frames_iter = ChunkedSlice::<BINARY_FRAME_LEN>::new(bytes);

	// here we allocate enough capacity (calculated by `EncodedReprInfo`)
	let mut dest = UnsafeBufWriteGuard::with_capacity(needed_capacity);

	// this is the loop that encodes and writes all the chunks to output buffer
	for _ in 0..frames {
		// SAFETY: we loop `frames` times, which is calculated correctly
		// by `EncodedReprInfo` (see its safety notes)
		let frame = unsafe { frames_iter.next_frame_unchecked() };

		// SAFETY: same as above, we will have enough bytes to write to,
		// calculated by `EncodedReprInfo`
		unsafe { encode_frame(frame, &mut dest) }
	}

	// this is where the remainder, if any, is encoded and stored
	if remainder > 0 {
		{
			// Refactoring out the inner closure is to avoid an edge case in lints
			// https://github.com/rust-lang/rust-clippy/issues/13134
			let do_thing = |remainder: &_| {
				// SAFETY: we calculated and preallocated the correct amount up front.
				// This closure is called only once, just after this declaration. I (vt)
				// have been extra cautious and put it in its own scope lol
				unsafe { encode_frame(remainder, &mut dest) }
			};

			// SAFETY: this will only run if there is any remainder (guarded by if statement)
			// All the full frames have been consumed by the previous loop, so if we do have
			// any remainder, it will be strictly under 4
			// This triggers, even with unsafe block on `encode_frame`, and additionally
			// the lint for unnecessary unsafe block triggers too. I think nested unsafe
			// is edge case that the authors of this lint have not thought about, hmm...
			unsafe { frames_iter.with_remainder_unchecked(do_thing) }
		}

		// remainder is amount of non padding bytes in the frame
		let padding_len = 4 - remainder;

		// SAFETY: remainder will always be 1, 2, or 3 (both strictly less than 4
		// as guaranteed by `EncodedReprInfo`, and greater than 0, by if condition),
		// so the result of the subtraction above, will always be 3, 2, or 1, which
		// is less than 85, so this will always be in bounds
		let ptr = unsafe { TABLE_ENCODER.as_ptr().add(padding_len) };

		// SAFETY: ptr is in bounds and valid (see above)
		let padding_char = unsafe { *ptr };

		// SAFETY: we calculated and preallocated the right amount, which if we
		// are here, would be because we needed to encode remainder, and allocation
		// calculation will have taken that into account would include this extra
		// byte that is needed
		unsafe { dest.write_bytes_const::<1>(&padding_char) }
	}

	// SAFETY: we calculated and requested the exact amount of memory we need,
	// and have written to all of it in the loop and if statement above
	let vec = unsafe { dest.into_full_vec() };

	// SAFETY: throughout the encoding process, we only pushed characters from the
	// encoding table, all of which are ASCII chars. If a string is valid ASCII,
	// it is also valid UTF-8
	unsafe {
		debug_assert!(str::from_utf8(&vec).is_ok(), "output bytes are valid utf-8");
		String::from_utf8_unchecked(vec)
	}
}

/// Decodes a slice of a Z85 string back into the source bytes
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
	// Either, there is 1 trailing byte, that is stritly less than 4, that
	// encodes the amount of padding added. Or, it's a different character that
	// doesn't decode to 0..=3 or none at all (in which case, we exit early)
	let (capacity, added_padding) = match remainder {
		0 => {
			// no padding was added
			(capacity, 0usize)
		}
		1 => {
			// the singular trailing byte that encodes how much padding was added

			// remainder is 1, so there will be at least 1 byte in the
			// slice, because duh (well technically there will be at least 6
			// bytes as established earlier, but I can still safetly subtract 1).
			// This will never overflow
			// TODO: could be unchecked sub?
			let one_shorter = bytes.len() - 1;

			// le ptr to the input slice
			let ptr = bytes.as_ptr();

			let byte = {
				// SAFETY: this points at the last byte, it is in bounds
				let last_byte_ptr = unsafe { ptr.add(one_shorter) };

				// SAFETY: since the ptr is in bounds and points
				// at last byte, it is safe to dereference
				unsafe { *last_byte_ptr }
			};

			// SAFETY:
			// - `ptr` points at the start of the input slice
			// - it's established above that subtracting by 1
			//   will not overflow, if we got into this match branch
			// - `one_shorter` is one less than the len of the input slice
			//    (which won't overflow)
			// this is just taking a subslice of the all the bytes except the last
			bytes = unsafe { slice::from_raw_parts(ptr, one_shorter) };

			let decoded = {
				// SAFETY: `byte` is of type u8, which has a range of 0..=255,
				// which will never overflow TABLE_DECODER as its len is 256
				let table_ptr = unsafe { TABLE_DECODER.as_ptr().add(byte.into_usize()) };

				// SAFETY: as established above, pointer above will not
				// index past end of TABLE_DECODER
				unsafe { *table_ptr }
			};

			let decoded = decoded.into_usize();
			let added_padding = if decoded < BINARY_FRAME_LEN {
				decoded
			} else {
				// invalid char in this context (too large / does not exist)
				// does not exist is 255 so it won't pass above check either
				return Err(DecodeError::InvalidChar)
			};

			// We established that if we got here, we need to decode at least 1
			// full frame. `added_padding` is lte 3, which is less than the
			// size of 1 full binary frame (4), so this won't overflow. If
			// added_padding is 0 for some reason, this returns the same values
			// as the 0 case (checked in unit test below)
			// TODO: this can be unchecked sub
			(capacity - added_padding, added_padding)
		}
		_n => {
			// 2 or 3 extra bytes at end of input, not valid in any scenario
			return Err(DecodeError::InvalidLength)
		}
	};

	// because frames >= 1, `excluding_last_frame` will be >= 0 (ie. will not underflow).
	let excluding_last_frame = frames - 1;

	let mut frames_iter = ChunkedSlice::<STRING_FRAME_LEN>::new(bytes);
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);

	// this loop goes over and decodees all the string chunks to output buffer
	for _ in 0..excluding_last_frame {
		// SAFETY: this loop loops `excluding_last_frame` times, which is the
		// amount of times it takes to loop through all the full chunks, except
		// the last one
		let frame = unsafe { frames_iter.next_frame_unchecked() };

		// Refactoring out the inner closure is to avoid an edge case in lints
		// https://github.com/rust-lang/rust-clippy/issues/13134
		let do_thing = |frame: &[_; 4]| {
			// SAFETY: we calculated/preallocated the exact amount of
			// memory we need up front, and we only loop one less amount
			// of times than the number of full frames, so we won't overflow
			unsafe { dest.write_bytes_const::<BINARY_FRAME_LEN>(frame.as_ptr()) }
		};

		// SAFETY: uhm, conservatively marked unsafe yay? lol
		// but as established above, we won't overflow
		unsafe { decode_frame(frame, do_thing)? }
	}

	// this is the last frame, and this frame gets decoded a bit specially.
	// We remove the amount of padding from the amount, so we can only write
	// the amount of actual data bytes into the output. We are still able to
	// preallocate exact capacity we need up front!

	// SAFETY: we do have one last frame left. if there was 1 remainder it was
	// decoded and removed from the end, if there was more remainder we returned
	// an error already, and if it were 0 then.. well, we have a perfect frame left.
	// After this, there will be no frames left.
	let frame = unsafe { frames_iter.next_frame_unchecked() };

	// This is the amount of bytes minus the padding bytes at the end, aka, the
	// amount of actual data bytes that were encoded.
	// - if 0 bytes of padding were added, this is whole frame and
	//   added_padding would be 0
	// - if 1 to 3 bytes of padding were added, this is correct, and would be
	//   1 to 3
	// - 4 or more here is not possible, since if it did, full frames would have
	//   been counted towards full frame count and strict remainder would be here
	//   (0..=3)

	// Because of all that explained above, this will also be in range of
	// 0 <= n < 4 (BINARY_FRAME_LEN), ie. will not overflow.
	let non_padding_bytes = BINARY_FRAME_LEN - added_padding;

	let do_thing = |frame: &[_; 4]| {
		// SAFETY: this writes the actual data bytes into the buffer. We
		// subtracted padding bytes from the number we write already, so we
		// write the rest of the buffer the perfect amount left
		unsafe { dest.write_bytes(frame.as_ptr(), non_padding_bytes) }
	};

	// SAFETY: conservatively marked unsafe function aha
	// (reasoning for other invariants detailed out above)
	unsafe { decode_frame(frame, do_thing)? }

	// SAFETY: We have consumed all the input bytes (calculated)
	debug_assert!(frames_iter.to_slice().is_empty(), "all bytes were consumed");

	// SAFETY: We have written the exact amount of bytes we preallocated (calculated)
	Ok(unsafe { dest.into_full_vec() })
}

/// Errors that can be encountered on decoding data (encoding data does not error)
// TODO: these errors could be improved.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DecodeError {
	/// Input data has invalid length
	#[error("invalid length")]
	InvalidLength,
	/// Invalid character in input data
	#[error("invalid character")]
	InvalidChar,
	/// One frame of data contains characters that are too large and would overflow
	#[error("correct characters, but incorrect combination that would cause overflow")]
	FrameOverflow
}

/// Various details about a slice and it's encoded output bytes, including
/// number of full frames, remainder, and how much capacity is needed to hold
/// all the encoded bytes
///
/// # Safety
///
/// All fields on this struct are marked `pub`, meaning anyone is allowed to
/// directly access and modify them. Don't accept any instances of this struct
/// from nontrusted sources, nor construct instances from raw data taken from
/// nontrusted sources.
///
/// On the contrary, you can trust and rely on the output directly from the
/// [`for_input_len`](EncodedReprInfo::for_input_len) associated function,
/// including in unsafe contexts. The body of this function is heavily
/// documented.
pub struct EncodedReprInfo {
	/// The amount of _full_ frames (eg. amount of full chunks of 4 bytes)
	pub frames: usize,
	/// The amount of remainder bytes, strictly less than 4 (frame size)
	pub remainder: usize,
	/// The amount of capacity required to fit all the encoded data into
	///
	/// This is calculated by summing up the following values:
	/// - Space needed for full frames is `frames * 5`, since every frame
	///   is a chunk of 4 bytes that gets encoded into a frame of 5 bytes
	/// - Space needed for remainder bytes:
	///   - If no remainder, then 0. Simple enough :p
	///     - This also implies that if the input does not need to be padded to
	///       a len that is a multiple of 4, no padding is needed. In this case,
	///       the numbers/calculations here are compliant with the [Z85 spec].
	///   - If there is remainder, it is `5 + 1`. The remainder bytes
	///     will be padded to a full frame of 4, then encoded as a full frame,
	///     yielding 5. Then, one extra byte is added onto the end to encode the
	///     amount of padding we have added (ex. 1 for 3 remainder bytes and 1
	///     padding).
	///
	/// [Z85 spec]: https://rfc.zeromq.org/spec/32
	pub needed_capacity: usize
}

impl EncodedReprInfo {
	/// Calculates the values
	///
	/// See documentation on [`EncodedReprInfo`] and on the individual fields
	/// for more information.
	#[inline]
	pub fn for_input_len(input_len: usize) -> Self {
		// right shift 2 is same as integer divide by 4,
		// to get the amount of full binary frames
		let frames = input_len >> 2;

		// binary AND with 0b11 (3) is the same as rem 4,
		// to get the amount of remainder bytes
		let remainder = input_len & 0b11;

		let needed_capacity = if remainder == 0 {
			// each frame of 4 is encoded to a frame of
			// 5 bytes of output. No padding needed case
			frames * 5
		} else {
			// `frames` is number of *whole* binary frames, so the remainder
			// is not included in this. adding 1 to allocate space for one more
			// frame containing the padded remainder
			let capacity = (frames + 1) * STRING_FRAME_LEN;

			// adding 1 more byte for the last byte that
			// encodes amount of padding added
			capacity + 1
		};

		Self { frames, remainder, needed_capacity }
	}
}

/// # Safety
///
/// Caller must guarantee dest is valid for at least `STRING_FRAME_LEN` bytes
/// to be written.
unsafe fn encode_frame(frame: &[u8; BINARY_FRAME_LEN], dest: &mut UnsafeBufWriteGuard) {
	let mut int = u32::from_be_bytes(*frame).into_usize();

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

	let table_ptr = TABLE_ENCODER.as_ptr();

	/// # Safety
	///
	/// The value stored in the variable that's passed into this macro must be
	/// within the range 0..=84, so that it can be used to index the encode table
	macro_rules! encode_byte_unsafe {
		($byte:ident) => {
			{
				// SAFETY: macro caller promises variable is within 0..=84,
				// which can be safetly used to index encode table (len 85)
				let byte_ptr = unsafe { table_ptr.add($byte) };

				// SAFETY: as described above, the pointer is valid to read from
				unsafe { *byte_ptr }
			}
		}
	}

	// SAFETY: these are calculated by modulo TABLE_ENCODER_LEN (85), which
	// guarantees the numbers are 0 <= n < TABLE_ENCODER_LEN (85),
	// and the length of TABLE_ENCODER is 85, so this won't overflow
	let encoded_frame = [
		// SAFETY: all the below macro invocations pass in variables whose value
		// is calculating by rem 85. so they will always be strictly less than 85
		encode_byte_unsafe!(byte1),
		encode_byte_unsafe!(byte2),
		encode_byte_unsafe!(byte3),
		encode_byte_unsafe!(byte4),
		encode_byte_unsafe!(byte5)
	];

	// SAFETY: caller guarantees that `dest` has at least
	// `STRING_FRAME_LEN` bytes left, and that writing this won't overflow.
	unsafe { dest.write_bytes_const::<STRING_FRAME_LEN>(encoded_frame.as_ptr()) }
}

/// # Safety
///
/// All possible inputs are sound. However, marking this function `unsafe` is
/// consistent with [`encode_frame`]. This is an internal function, so doesn't
/// matter too much.
unsafe fn decode_frame<F>(frame: &[u8; STRING_FRAME_LEN], f: F) -> Result<(), DecodeError>
where
	F: FnOnce(&[u8; BINARY_FRAME_LEN])
{
	let [byte1, byte2, byte3, byte4, byte5] = *frame;
	let table_ptr = TABLE_DECODER.as_ptr();

	macro_rules! decode_byte_unsafe {
		($byte:ident) => {
			// SAFETY: caller promises that `$byte` is within range 0..=255,
			// and the decoding table is len 256, so this will not be out of bounds
			let ptr = unsafe { table_ptr.add($byte.into_usize()) };

			// SAFETY: as established above, the ptr is within
			// bounds and safe to dereference
			let $byte = unsafe { *ptr };
			if $byte == NONE { return Err(DecodeError::InvalidChar) }
		}
	}

	// SAFETY: the byte values provided are bytes, so will guaranteed
	// in the range 0..=255 (as larger values are not even representable)
	// Additionally, if this comes back as Some from TABLE_DECODER, it is guaranteed
	// to be 0 <= n <= 84, since there are no Some(n) values outside this range.
	decode_byte_unsafe!(byte1);
	decode_byte_unsafe!(byte2);
	decode_byte_unsafe!(byte3);
	decode_byte_unsafe!(byte4);
	decode_byte_unsafe!(byte5);

	// A string frame containing all valid z85 chars, can still overflow u32
	// (u32 max is 4.294.967.295, but the result of this operation can be 4.437.053.124).
	// However it cannot overflow u64 (u64 max is 18.446.744.073.709.551.616).
	// So we decode in a u64 first, and then we check for overflow past u32's
	// bits and error if so
	let mut int = byte1.into_u64();

	int *= TABLE_ENCODER_LEN.into_u64();
	int += byte2.into_u64();

	int *= TABLE_ENCODER_LEN.into_u64();
	int += byte3.into_u64();

	int *= TABLE_ENCODER_LEN.into_u64();
	int += byte4.into_u64();

	int *= TABLE_ENCODER_LEN.into_u64();
	int += byte5.into_u64();

	if int >> u32::BITS != 0 { return Err(DecodeError::FrameOverflow) }

	let decoded_frame = u32::to_be_bytes(int.into_u32_lossy());
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
			assert_eq!(decoded, Err(DecodeError::FrameOverflow))
		}
	}
}
