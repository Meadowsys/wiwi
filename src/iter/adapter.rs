use super::Iter;

pub trait AdaptStd {
	fn to_wiwi_iter(&mut self) -> StdIterAdapter<&mut Self>;
	fn into_wiwi_iter(self) -> StdIterAdapter<Self>
	where
		Self: Sized;
}

pub struct StdIterAdapter<I> {
	inner: I
}

impl<I: Iterator> AdaptStd for I {
	fn to_wiwi_iter(&mut self) -> StdIterAdapter<&mut Self> {
		StdIterAdapter { inner: self }
	}

	fn into_wiwi_iter(self) -> StdIterAdapter<Self>
	where
		Self: Sized
	{
		StdIterAdapter { inner: self }
	}
}

impl<I: Iterator> Iter for StdIterAdapter<I> {
	type Item = I::Item;
	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}
}
