//! h

use crate::prelude::*;

/// h
#[inline]
pub fn h() -> String {
	"h".into()
}

/// H
// shut
#[allow(non_snake_case)]
#[inline]
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
	// shut
	#[allow(non_snake_case)]
	pub fn H() {
		assert_eq!(super::H(), "H", "H should be H");
	}
}
