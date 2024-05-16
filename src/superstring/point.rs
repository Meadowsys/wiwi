use std::cmp::{ PartialEq, PartialOrd, Eq, Ord, Ordering };
use std::fmt::{ self, Display, Debug, Formatter };

pub struct Point {
	row: u32,
	col: u32
}

impl Point {
	pub fn new(row: u32, col: u32) -> Self {
		Self { row, col }
	}

	pub fn min_point() -> Self {
		Self { row: 0, col: 0 }
	}

	pub fn max_point() -> Self {
		Self { row: u32::MAX, col: u32::MAX }
	}

	pub fn is_zero(&self) -> bool {
		self.row | self.col == 0
	}

	pub fn traverse(&self, traversal: &Self) -> Self {
		// superstring has checked_add function that essentially
		// performs saturating add
		if traversal.row == 0 {
			Self::new(self.row, self.col.saturating_add(traversal.col))
		} else {
			Self::new(self.row.saturating_add(traversal.row), traversal.col)
		}
	}

	pub fn traversal(&self, start: &Self) -> Self {
		// according to here https://en.cppreference.com/w/cpp/language/operator_arithmetic
		// cpp subtraction wraps on underflow
		if self.row == start.row {
			Self::new(0, self.col.wrapping_sub(start.col))
		} else {
			Self::new(self.row.wrapping_sub(start.row), self.col)
		}
	}
}

impl Debug for Point {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Display::fmt(self, f)
	}
}

impl Display for Point {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let Self { row, col } = self;
		write!(f, "({row}, {col})")
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		self.row == other.row && self.col == other.col
	}
}

impl Eq for Point {}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Point {
	fn cmp(&self, other: &Point) -> Ordering {
		match self.row.cmp(&other.row) {
			Ordering::Equal => { self.col.cmp(&other.col) }
			other => { other }
		}
	}
}
