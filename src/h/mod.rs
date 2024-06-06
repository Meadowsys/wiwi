//! h

/// h
pub fn h() -> String {
	"h".into()
}

/// H
#[allow(non_snake_case)] // shut
pub fn H() -> String {
	"H".into()
}

#[cfg(test)]
mod tests {
	#[test]
	pub fn h() {
		assert_eq!(super::h(), "h", "h should be h");
	}

	#[test]
	#[allow(non_snake_case)] // shut
	pub fn H() {
		assert_eq!(super::H(), "H", "H should be H");
	}
}
