#[inline]
fn get_byte_count_unsigned_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	let ptr = bytes.as_ptr();

	for i in (1..BYTES).rev() {
		// simple! just return the first byte (including the byte) where
		// its not all 0s. Iter stops after offset 1 because we'll always
		// return at least 1 byte, so no point in checking last byte.

		// SAFETY: `ptr` is to an array of size `BYTES` and `i` from loop is
		// within `0..BYTES`, so this offset is in bounds and valid for reads
		let ptr = unsafe { ptr.add(i) };

		// SAFETY: as established above `ptr` is valid for reads
		let byte = unsafe { *ptr };
		if byte != 0 { return (i + 1) as _ }
	}

	// everything is empty? just return the minimum
	1
}

#[inline]
fn get_byte_count_signed_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	debug_assert!(BYTES > 0);

	let ptr = bytes.as_ptr();

	let sign_bit = {
		// SAFETY: `ptr` is to an array of size `BYTES`, where `BYTES` is gt 0,
		// so offsetting ptr by `BYTES - 1` is in bounds and valid for reads
		let ptr = unsafe { ptr.add(BYTES - 1) };

		// SAFETY: `ptr` is valid for reads as established above
		let byte = unsafe { *ptr };

		byte >> 7
	};

	// byte that has empty data and can (probably) be safely discarded.
	// if negative, all bits 1, if positive, all bits 0
	let empty_byte = if sign_bit == 0 { 0 } else { u8::MAX };

	for i in (0..BYTES).rev() {
		let byte = {
			// SAFETY: `ptr` is to an array of size `BYTES` and `i` from loop is
			// within `0..BYTES`, so this offset is in bounds and valid for reads
			let ptr = unsafe { ptr.add(i) };

			// SAFETY: as established above this offset ptr is valid for reads
			unsafe { *ptr }
		};

		// the following could be shortened to a one liner...
		// but this absolutely sucks for readability/maintainability, so nah
		// if byte != empty_byte { return (i + 1) as u8 + (byte >> 7 != sign_bit) as u8 }

		if byte == empty_byte {
			// byte is full of 1s if negative, or full of 0s if positive
			// this byte can (probably) be safely discarded; continue
		} else if byte >> 7 == sign_bit {
			// sign bit is the same, return up to / including this byte
			// iter range is 0 to BYTES - 1 (inclusive), so this return range
			// will be 1 to BYTES (inclusive), which is correct
			return (i + 1) as _
		} else {
			// sign bit is different, return this byte and one more after it.
			// if the next byte would have the wrong sign, it would have returned
			// already in the previous branch. This won't ever overflow because
			// the first byte will not have a different sign (as... itself),
			// so will never reach here
			return (i + 2) as _
		}
	}

	// everything is empty? just return the minimum
	1
}
