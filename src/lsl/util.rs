use crate::id::IDGenerator;
use spin_sleep::SpinSleeper;
use std::{ cell::RefCell, num::NonZeroU64, time::Duration };

/// Returns next ID. Guaranteed unique within the thread.
pub(super) fn next_id() -> NonZeroU64 {
	thread_local! {
		static GENERATOR: RefCell<(IDGenerator, SpinSleeper)> = {
			RefCell::new((IDGenerator::new(), SpinSleeper::default()))
		}
	}

	const SLEEP_DURATION: Duration = Duration::from_micros(250);

	GENERATOR.with_borrow_mut(|(gen, sleeper)| {
		loop {
			if let Some(id) = gen.next() {
				return id.as_unsigned_nonzero()
			}
			sleeper.sleep(SLEEP_DURATION);
		}
	})
}
