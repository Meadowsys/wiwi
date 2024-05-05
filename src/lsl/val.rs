//! Stores values and returns references to them.

use super::*;
use hashbrown::HashMap;
use std::cell::RefCell;

thread_local! {
	static VALUES: RefCell<HashMap<u64, String>> = {
		RefCell::new(HashMap::with_capacity(128))
	}
}

pub fn val<T: ty::Type, I: IntoVal<T>>(val: I) -> Val<T> {
	let id = util::next_id();
	let val = val.into_value();
	let _ty = PhantomData;
	let _st = PhantomData;

	VALUES.with_borrow_mut(|values| {
		values.insert(id, val);
	});

	Val { id, _ty, _st }
}

#[derive(Clone, Copy)]
pub struct Val<T> {
	id: u64,
	_ty: PhantomData<T>,
	_st: SingleThreadMarker
}

pub trait IntoVal<T> {
	fn into_value(self) -> String;
}
