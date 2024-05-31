use crate::chainer::{ IntoChainer, SliceBoxChain };
use rand::{ Rng, seq::SliceRandom, thread_rng };
use rand::distributions::uniform::SampleRange;
use std::fmt;
use std::num::NonZeroUsize;

// TODO: find `pub` in this file and reasses all of it lol

#[derive(Clone)]
pub struct Board {
	w: NonZeroUsize,
	h: NonZeroUsize,
	// 0  1  2  3
	// 4  5  6  7
	// 8  9  10 11
	// 12 13 14 15
	board: Box<[Cell]>
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Cell {
	inner: u8
}

impl Board {
	pub fn new(w: NonZeroUsize, h: NonZeroUsize) -> Self {
		// SAFETY: zeroed is valid bit pattern for u8, and
		// Cell has repr(transparent) to u8, so this is valid.
		// a Cell with zero bit pattern is not revealed, not a mine, with zero surrounding
		// mines (board is a valid board, even if a boring one :p)
		let board = unsafe {
			SliceBoxChain::new_zeroed(w.get() * h.get())
				.assume_init()
				.into_inner()
		};
		Self { w, h, board }
	}

	pub fn new_random_mines(w: NonZeroUsize, h: NonZeroUsize, mines: usize) -> Self {
		let mut board = Self::new(w, h);
		board.add_random_mines(mines);
		board
	}

	// TODO: ideally this function does not need to exist
	pub fn force_update_counts(&mut self) {
		// TODO: can be optimised maybe? skip cells with a mine?
		for r in 0..self.h.get() {
			for c in 0..self.w.get() {
				unsafe {
					let mut mines_count = 0u8;

					let go_up = r > 0;
					let go_down = r < self.h.get() - 1;
					let go_left = c > 0;
					let go_right = c < self.w.get() - 1;

					if go_up {
						let r = r - 1;
						mines_count += self.get_coords_unchecked(r, c).is_mine() as u8;

						if go_left {
							let c = c - 1;
							mines_count += self.get_coords_unchecked(r, c).is_mine() as u8;
						}

						if go_right {
							let c = c + 1;
							mines_count += self.get_coords_unchecked(r, c).is_mine() as u8;
						}
					}

					if go_down {
						let r = r + 1;
						mines_count += self.get_coords_unchecked(r, c).is_mine() as u8;

						if go_left {
							let c = c - 1;
							mines_count += self.get_coords_unchecked(r, c).is_mine() as u8;
						}
						if go_right {
							let c = c + 1;
							mines_count += self.get_coords_unchecked(r, c).is_mine() as u8;
						}
					}

					if go_left {
						let c = c - 1;
						mines_count += self.get_coords_unchecked(r, c).is_mine() as u8;
					}

					if go_right {
						let c = c + 1;
						mines_count += self.get_coords_unchecked(r, c).is_mine() as u8;
					}

					self.get_coords_unchecked_mut(r, c).set_surrounding_count_unchecked(mines_count);
				}
			}
		}
	}

	/// Clears the board in place.
	///
	/// This removes all mines from every cell, unreveals all cells, and updates
	/// surrounding cell mine counts accordingly. It doesn't touch the board's
	/// size (if you want a different-sized board, you should create a new
	/// instance with the new dimensions).
	pub fn clear(&mut self) {
		unsafe { self.board_ptr_mut().write_bytes(0, self.board.len()) }
	}

	pub unsafe fn offset_of_unchecked(&self, r: usize, c: usize) -> usize {
		self.debug_assert_in_bounds(r, c);
		self.__offset(r, c)
	}

	pub fn add_random_mines(&mut self, mines: usize) {
		let mut rng = thread_rng();
		(1..self.board.len())
			.rfold(
				self.board.iter_mut().collect::<Vec<_>>().into_chainer(),
				|board, i| unsafe { board.swap_unchecked(i, (0..=i).sample_single(&mut rng)) }
			)
			.nonchain_inner()
			.into_iter()
			.filter(|cell| !cell.is_mine())
			.take(mines)
			.for_each(|cell| cell.place_mine());
		self.force_update_counts();
	}

	#[inline]
	unsafe fn board_ptr(&self) -> *const Cell {
		self.board.as_ptr()
	}

	#[inline]
	unsafe fn board_ptr_mut(&mut self) -> *mut Cell {
		self.board.as_mut_ptr()
	}

	pub unsafe fn get_coords_unchecked(&self, r: usize, c: usize) -> &Cell {
		&*self.board_ptr().add(self.offset_of_unchecked(r, c))
	}

	pub unsafe fn get_coords_unchecked_mut(&mut self, r: usize, c: usize) -> &mut Cell {
		&mut *self.board_ptr_mut().add(self.offset_of_unchecked(r, c))
	}

	#[inline(always)]
	pub fn debug_assert_in_bounds(&self, r: usize, c: usize) {
		debug_assert!(r < self.h.get());
		debug_assert!(c < self.w.get());
		debug_assert!(
			unsafe { self.__offset(r, c) < self.board.len() },
			"invalid state: w = {w}, h = {h}, board len = {len} (should be w * h)",
			w = self.w,
			h = self.h,
			len = self.board.len()
		);
	}

	#[inline(always)]
	unsafe fn __offset(&self, r: usize, c: usize) -> usize {
		(r * self.w.get()) + c
	}
}

impl Cell {
	#[inline]
	pub fn is_mine(&self) -> bool {
		(self.inner >> 1) & 1 != 0
	}

	#[inline]
	pub fn is_revealed(&self) -> bool {
		self.inner & 1 != 0
	}

	#[inline]
	pub fn place_mine(&mut self) {
		self.inner |= 1 << 1;
	}

	#[inline]
	pub fn reveal(&mut self) -> bool {
		self.inner |= 1;
		self.is_mine()
	}

	#[inline]
	pub unsafe fn set_surrounding_count_unchecked(&mut self, count: u8) {
		debug_assert!(count <= 8);
		self.inner |= count << 2;
	}

	#[inline]
	pub fn set_surrounding_count_checked(&mut self, count: u8) -> Option<()> {
		(count <= 8).then(|| unsafe { self.set_surrounding_count_unchecked(count) })
	}

	#[inline]
	pub fn set_surrounding_count(&mut self, count: u8) {
		self.set_surrounding_count_checked(count)
			.expect("count was too large (greater than 8)")
	}

	#[inline]
	pub fn surrounding_count(&self) -> u8 {
		self.inner >> 2
	}
}

impl fmt::Debug for Cell {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.is_mine() {
			write!(f, "X")
		} else {
			write!(f, "{}", self.surrounding_count())
		}
	}
}

impl fmt::Debug for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// TODO: how do I avoid trailing newline in an elegant way?
		for r in 0..self.h.get() {
			for c in 0..self.w.get() {
				unsafe {
					write!(f, "  {:?}", self.get_coords_unchecked(r, c))?;
				}
			}
			writeln!(f)?;
		}

		Ok(())
	}
}
