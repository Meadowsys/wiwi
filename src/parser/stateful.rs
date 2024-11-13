use super::Result;

pub trait ParserStateful<D, O, E = ()> {
	fn parse_stateful(&mut self, data: D) -> Result<D, O, E>;
}

impl<T, D, O, E> ParserStateful<D, O, E> for T
where
	T: super::Parser<D, O, E>
{
	#[inline]
	fn parse_stateful(&mut self, data: D) -> Result<D, O, E> {
		self.parse(data)
	}
}
