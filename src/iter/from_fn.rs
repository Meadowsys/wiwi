use super::Iter;

#[repr(transparent)]
pub struct FromFn<F> {
	inner: F
}

pub fn from_fn<T, F>(f: F) -> FromFn<F>
where
	F: FnMut() -> Option<T>
{
	FromFn { inner: f }
}

impl<T, F> Iter for FromFn<F>
where
	F: FnMut() -> Option<T>
{
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<T> {
		(self.inner)()
	}
}
