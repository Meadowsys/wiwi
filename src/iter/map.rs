use super::{ Iter, Peekable, SizeHint };

pub struct Map<I, F> {
	iter: I,
	f: F
}

impl<I, F, O> Map<I, F>
where
	I: Iter,
	F: FnMut(I::Item) -> O
{
	pub(super) fn new(iter: I, f: F) -> Self {
		Self { iter, f }
	}

	pub fn into_inner(self) -> (I, F) {
		let Self { iter, f } = self;
		(iter, f)
	}
}

impl<I, F, O> Iter for Map<I, F>
where
	I: Iter,
	F: FnMut(I::Item) -> O
{
	type Item = O;

	fn next(&mut self) -> Option<O> {
		self.iter.next().map(&mut self.f)
	}

	fn size_hint(&self) -> SizeHint {
		self.iter.size_hint()
	}
}


impl<I, F, O> Peekable for Map<I, F>
where
	I: Peekable,
	F: FnMut(I::Item) -> O
{
	type PeekItem = I::PeekItem;

	fn peek(&mut self) -> Option<&I::PeekItem> {
		self.iter.peek()
	}
}









// pub struct MapPeekable<I, F, P> {
// 	map: Map<I, F>,
// 	peek_f: P
// }

// impl<'h, I, F, O, P> MapPeekable<I, F, P>
// where
// 	I: Peekable,
// 	F: FnMut(I::Item) -> O
// {
// 	pub(super) fn new<Po>(iter: I, f: F, peek_f: P) -> Self
// 	where
// 		P: FnMut(I::PeekItem) -> Po,
// 		Po: 'h
// 	{
// 		let map = Map::new(iter, f);
// 		Self { map, peek_f }
// 	}

// 	pub fn into_inner(self) -> (I, F, P) {
// 		let Self { map, peek_f } = self;
// 		let Map { iter, f } = map;
// 		(iter, f, peek_f)
// 	}
// }

// impl<I, F, O, P> Iter for MapPeekable<I, F, P>
// where
// 	I: Peekable,
// 	F: FnMut(I::Item) -> O
// {
// 	type Item = O;

// 	fn next(&mut self) -> Option<O> {
// 		self.map.next()
// 	}

// 	fn size_hint(&self) -> SizeHint {
// 		self.map.size_hint()
// 	}
// }

// impl<I, F, O, P, Po> Peekable for MapPeekable<I, F, P>
// where
// 	I: Peekable,
// 	F: FnMut(I::Item) -> O,
// 	P: FnMut(&I::PeekItem) -> &Po
// {
// 	type PeekItem = Po;

// 	fn peek(&mut self) -> Option<&Po> {
// 		self.map.iter.peek().map(&mut self.peek_f)
// 	}
// }
