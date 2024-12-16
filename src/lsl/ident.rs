use crate::prelude::*;
use crate::rc::RcThread;

/// Identifier incrementer for generating unique LSL identifiers
pub struct IdentIncrementer {
	next: RcThread<cell::Cell<u64>>
}

impl IdentIncrementer {
	fn new() -> Self {
		Self { next: RcThread::from_value(cell::Cell::new(0)) }
	}

	fn next(&self) -> u64 {
		let next = self.next.as_value_ref().get();
		self.next.as_value_ref().set(next.checked_add(1).unwrap());
		next
	}

	fn mk_owned_ref(&self) -> Self {
		Self { next: RcThread::clone(&self.next) }
	}
}

impl Clone for IdentIncrementer {
	fn clone(&self) -> Self {
		self.mk_owned_ref()
	}
}

#[derive(Clone)]
pub struct Ident {
	ident: u64
}

impl Ident {
	fn to_u64(&self) -> u64 { self.ident }
}
