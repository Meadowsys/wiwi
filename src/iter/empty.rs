use std::marker::PhantomData;
use super::{ Iter, SizeHintImpl, SizeHintMarker };

pub struct Empty<T> {
	nothinglol: PhantomData<T>
}

pub fn empty<T>() -> Empty<T> {
	Empty { nothinglol: PhantomData }
}

impl<T> Iter for Empty<T> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<T> {
		None
	}

	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		SizeHintImpl::hard(0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let mut empty = super::empty::<String>();
		assert_eq!(empty.next(), None);
		assert_eq!(empty.next(), None);
		assert_eq!(empty.next(), None);
		assert_eq!(empty.next(), None);
		assert_eq!(empty.next(), None);
		assert_eq!(empty.next(), None);
		assert_eq!(empty.next(), None);
		// ...
	}

	#[test]
	fn size_hint() {
		let empty = super::empty::<String>();
		assert_eq!(empty.size_hint(), unsafe { SizeHintImpl::hard(0) });
		// ... yeah, it's empty
	}
}
