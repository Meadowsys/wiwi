use super::{ Iter, SizeHint };

pub trait Peekable: Iter {
	type PeekItem;
	fn peek(&mut self) -> Option<&Self::PeekItem>;
}

pub struct Peek<I, T> {
	iter: I,
	peeked: Option<Option<T>>
}

impl<I, T> Peek<I, T>
where
	I: Iter<Item = T>
{
	pub(super) fn new(iter: I) -> Self {
		Self { iter, peeked: None }
	}

	pub fn into_inner(self) -> (I, Option<Option<T>>) {
		let Self { iter, peeked } = self;
		(iter, peeked)
	}
}

impl<I, T> Iter for Peek<I, T>
where
	I: Iter<Item = T>
{
	type Item = I::Item;

	fn next(&mut self) -> Option<I::Item> {
		self.peeked.take().unwrap_or_else(|| self.iter.next())
	}
}

impl<I, T> Peekable for Peek<I, T>
where
	I: Iter<Item = T>,
{
	type PeekItem = I::Item;

	fn peek(&mut self) -> Option<&I::Item> {
		self.peeked.get_or_insert_with(|| self.iter.next()).as_ref()
	}
}
