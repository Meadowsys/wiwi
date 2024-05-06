use crate::{ hex::encode_hex, id };
use spin_sleep::SpinSleeper;
use std::{ num::NonZeroU64, time::Duration };

pub struct IDGenerator {
	inner_gen: id::IDGenerator,
	sleeper: SpinSleeper
}

impl IDGenerator {
	pub fn new() -> Self {
		Self {
			inner_gen: id::IDGenerator::new(),
			sleeper: SpinSleeper::default()
		}
	}
	pub fn next(&mut self) -> Identifier {
		const SLEEP_DURATION: Duration = Duration::from_micros(250);

		loop {
			if let Some(id) = self.inner_gen.next() {
				return Identifier { inner: id.as_unsigned_nonzero() }
			}
			self.sleeper.sleep(SLEEP_DURATION)
		}
	}
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
	inner: NonZeroU64
}

impl Identifier {
	pub fn as_u64(self) -> u64 {
		self.inner.get()
	}

	pub fn as_nonzero_u64(self) -> NonZeroU64 {
		self.inner
	}

	pub fn as_string(self) -> String {
		encode_hex(&self.inner.get().to_le_bytes())
	}
}
