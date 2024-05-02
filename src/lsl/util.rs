use std::cell::RefCell;

pub fn next_id() -> u64 {
	thread_local! {
		static NEXT_ID: RefCell<u64> = const { RefCell::new(0) };
	}

	NEXT_ID.with_borrow_mut(|next| {
		let id = *next;
		*next += 1;
		id
	})
}
