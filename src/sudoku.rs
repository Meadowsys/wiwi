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
			<&SliceRefChain<_>>::from(&game as &[_])
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

// TODO: offset table can be (space) optimised further:
// right now its 2187 bytes. but instead, we can store every row, every col, and
// every cell group, which would take up 3 * 81 bytes. Then in another table, we
// could store a flattened array of cells, each containing an array (tuple?) with
// index to the row, col, and cell group, which takes another 3 * 81 bytes. This
// is total 486 bytes. If I really wanted I could get into bit packing...

type OffsetTable = [[[u8; 9]; 3]; 81];

/// usage: `OFFSET_TABLE[board_offset][a][b]` where:
///
/// - row: 0-indexed row, 0 to 8 inclusive
/// - col: 0-indexed col, 0 to 8 inclusive
/// - `board_offset`: (row * 9) + col
/// - `a`: 0 for offsets of cells in the same row, 1 for same but col, 2 for
///   same but group
/// - `b`: index 0 to 8, to retrieve offset of a cell in the same row/col/group
static OFFSET_TABLE: OffsetTable = generate_offsets();

const fn generate_offsets() -> OffsetTable {
	let mut table = [[[0u8; 9]; 3]; 81];

	// TODO: convert to for loops, once that's stabilised in const fns
	let mut row = 0;
	while row < 9 {
		let mut col = 0;
		while col < 9 {
			let board_offset = (row * 9) + col;
			// + 0 1 2 3 ...-> col
			// 0 0 1 2 3
			// 1 4 5 6 7
			// 2 8 9 ...
			// 3
			// ...
			// ↓
			// row
			//
			// offset = (row * 9) + col
			// row and col are 0 indexed

			let mut i = 0;
			while i < 9 {
				table[board_offset][0][i] = ((row * 9) + i) as u8;
				i += 1;
			}

			let mut i = 0;
			while i < 9 {
				table[board_offset][1][i] = ((i * 9) + col) as u8;
				i += 1;
			}

			let row_group = match row {
				0..=2 => { 0usize..=2 }
				3..=5 => { 3..=5 }
				6..=8 => { 6..=8 }
				_ => { unreachable!() }
			};

			let col_group = match col {
				0..=2 => { 0usize..=2 }
				3..=5 => { 3..=5 }
				6..=8 => { 6..=8 }
				_ => { unreachable!() }
			};

			let mut i = 0;

			let mut row_offset = *row_group.start();
			while row_offset <= *row_group.end() {
				let mut col_offset = *col_group.start();
				while col_offset <= *col_group.end() {
					table[board_offset][2][i] = ((row_offset * 9) + col_offset) as u8;
					i += 1;

					col_offset += 1;
				}

				row_offset += 1;
			}

			col += 1;
		}

		row += 1;
	}

	table
}
