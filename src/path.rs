use std::{ slice, str };

pub const SEP: char = '/';

pub fn basename(path: &str) -> &str {
	let mut chars = path.char_indices().rev().peekable();

	// skip all the trailing slashes
	let mut end = path.len();
	while let Some((i, SEP)) = chars.peek() {
		end = *i;
		let _ = chars.next();
	}

	// skip until we hit another slash (or end)
	let mut start = end;
	for (i, char) in chars {
		if char == SEP {
			// hit another slash, we're done
			// SAFETY: start and end are both values originating from
			// `char_indices()`, so they are valid
			return unsafe { substring_unchecked(path, start, end) }
		}
		start = i;
	}

	// there were no more slashes
	// SAFETY: see above safety comment
	unsafe { substring_unchecked(path, start, end) }
}

unsafe fn substring_unchecked(s: &str, start: usize, end: usize) -> &str {
	debug_assert!(end <= s.len());
	debug_assert!(s.is_char_boundary(start));
	debug_assert!(s.is_char_boundary(end));

	let ptr = s.as_ptr();
	let len = end - start;

	str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(start), len))
}

#[cfg(test)]
mod tests {
	#[test]
	fn basename() {
		// (path, expected)
		let strs = [
			("path", "path"),
			("/path/kiwin/wiwi", "wiwi"),
			("/path/kiwin/wiwi///////e/", "e"),
			("/path/kiwin/wiwi///////", "wiwi")
		];

		for (path, expected) in strs {
			assert_eq!(super::basename(path), expected);
		}
	}
}
