use std::marker::PhantomData;
use super::Iter;

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
}
