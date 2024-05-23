use crate::chainer::SliceRefChain;
use crate::iter::*;
use std::mem::MaybeUninit;

pub struct Board {
	inner: [CellValue; 81]
}

/// Layout of the bitfield: `Gxxxxxx987654321`, where the digits 1..=9 are
/// marking values of 1 to 9, and G is marker for if this value is a given value
/// (part of the initial puzzle). If it is, only one of 1 to 9 may be marked,
/// which is the given value. If it isn't a given value, any combination may be
/// toggled on or off, to indicate possible values.
struct CellValue {
	/// bitfield
	bitfield: u16
}


impl Board {
	pub fn new(game: [[u8; 9]; 9]) -> Self {
		debug_assert_eq!({
			SliceRefChain::<_>::from(&game as &[_])
				.flatten()
				.as_slice()
				.len()
		}, 81);

		let mut board = MaybeUninit::<[CellValue; 81]>::uninit();
		let board_ptr = board.as_mut_ptr() as *mut CellValue;

		for (r, arr) in game.into_iter().enumerate() {
			for (c, cell) in arr.into_iter().enumerate() {
				let offset = (r * 9) + c;

				if cell != 0 {
					// TODO: better error handling
					assert!(cell >= 1);
					assert!(cell <= 9);

					unsafe {
						board_ptr
							.add(offset)
							.write(CellValue::from_given_u8_unchecked(cell));
					}
				} else {
					unsafe {
						board_ptr
							.add(offset)
							.write(CellValue::new_conservatively_marked_nongiven());
					}
				}
			}
		}

		let inner = unsafe { board.assume_init() };
		Self { inner }
	}
}

impl CellValue {
	/// # Safety
	///
	/// The provided value must be within 1..=9
	unsafe fn from_given_u8_unchecked(val: u8) -> Self {
		Self { bitfield: (1 << (val - 1)) | 0x8000 }
	}

	/// TODO: find a better name than "nongiven" to the values that aren't part
	/// of the initial puzzle lol
	fn new_conservatively_marked_nongiven() -> Self {
		// last 9 bits filled
		Self { bitfield: 0b111111111 }
	}

	unsafe fn is_given(&self) -> bool {
		self.bitfield >> 15 == 1
	}

	/// # Safety
	///
	/// `self` must not be a given value, and `val` must be within 1..=9
	unsafe fn mark_possible_unchecked(&mut self, val: u8) {
		self.bitfield |= 1 << (val - 1)
	}

	/// # Safety
	///
	/// `self` must not be a given value, and `val` must be within 1..=9
	unsafe fn unmark_possible_unchecked(&mut self, val: u8) {
		self.bitfield &= !(1 << (val - 1))
	}

	/// # Safety
	///
	/// `self` must be a given value
	unsafe fn value_of_given(&self) -> u8 {
		debug_assert!(
			self.bitfield.count_ones() == 2,
			"a given value should only have G bit and one value bit set"
		);

		let mut value = 1;
		let mut acc = self.bitfield;

		loop {
			debug_assert!(value <= 9, "bitfield somehow unmarked (not given value?)");
			if acc & 1 == 1 { return value }

			acc >>= 1;
			value += 1;
		}
	}

	unsafe fn ungiven_possible_values_iter(&self) -> CellUngivenValuesIter {
		CellUngivenValuesIter::new(self.bitfield)
	}
}

struct CellUngivenValuesIter {
	value: u8,
	acc: u16
}

impl CellUngivenValuesIter {
	fn new(bitfield: u16) -> Self {
		Self { value: 1, acc: bitfield }
	}
}

impl Iter for CellUngivenValuesIter {
	type Item = u8;
	fn next(&mut self) -> Option<u8> {
		while self.value <= 9 {
			let acc = self.acc;
			let value = self.value;

			self.value += 1;
			self.acc >>= 1;
			if acc & 1 == 1 { return Some(value) }
		}

		None
	}
}

/// Indexing with board offset (`0..81`) will yield offsets to the other tables.
/// 0 being row offsets, 1 being col offsets, 2 being group offsets
// const OFFSET_TABLE: OffsetTable = generate_offsets();
const OFFSET_TABLE: [[u8; 3]; 81] = generate_offsets();

const ROW_OFFSETS: [[u8; 9]; 9] = generate_rows_table();
const COL_OFFSETS: [[u8; 9]; 9] = generate_cols_table();
const GROUP_OFFSETS: [[u8; 9]; 9] = generate_groups_table();

const fn generate_offsets() -> [[u8; 3]; 81] {
	let mut table = [[0u8; 3]; 81];

	let mut row = 0usize;
	while row < 9 {
		let mut col = 0usize;
		while col < 9 {
			let offset = (row * 9) + col;

			table[offset][0] = row as _;
			table[offset][1] = col as _;

			let g_row = row / 3;
			let g_col = col / 3;
			let g_offset = (g_row * 3) + g_col;
			table[offset][2] = g_offset as _;

			col += 1;
		}

		row += 1;
	}

	table
}

const fn generate_rows_table() -> [[u8; 9]; 9] {
	let mut table = [[0u8; 9]; 9];

	let mut row = 0usize;
	while row < 9 {
		let mut col = 0usize;
		while col < 9 {
			let offset = (row * 9) + col;
			table[row][col] = offset as _;

			col += 1;
		}

		row += 1;
	}

	table
}

const fn generate_cols_table() -> [[u8; 9]; 9] {
	let mut table = [[0u8; 9]; 9];

	let mut row = 0usize;
	while row < 9 {
		let mut col = 0usize;
		while col < 9 {
			let offset = (row * 9) + col;
			table[col][row] = offset as _;

			col += 1;
		}

		row += 1;
	}

	table
}

const fn generate_groups_table() -> [[u8; 9]; 9] {
	let mut table = [[0u8; 9]; 9];

	let groups = [0u8..3, 3..6, 6..9];

	let mut row = 0usize;
	while row < 3 {
		let mut col = 0usize;
		while col < 3 {
			let root_offset = (row * 3) + col;

			let row_group = &groups[row];
			let col_group = &groups[col];

			let mut row = row_group.start;
			while row < row_group.end {
				let mut col = col_group.start;
				while col < col_group.end {
					let board_offset = (row * 9) + col;

					let r = row - row_group.start;
					let c = col - col_group.start;
					let offset = (r * 3) + c;
					table[root_offset][offset as usize] = board_offset;

					col += 1;
				}

				row += 1;
			}

			col += 1;
		}

		row += 1;
	}

	table
}
