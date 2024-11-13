use crate::prelude_std::*;
use crate::num::*;
use super::{ Error, Parser as _, Result, Success, stateless };

pub trait ParserStateful<D, O, E = ()> {
	fn parse_stateful(&mut self, data: D) -> Result<D, O, E>;
}

#[inline]
pub fn delimited<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>(
	parser_before: PBefore,
	parser: P,
	parser_after: PAfter
) -> Delimited<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	P: ParserStateful<D, O, E>,
	PBefore: ParserStateful<D, OBefore, EBefore>,
	PAfter: ParserStateful<D, OAfter, EAfter>,
	EBefore: Into<E>,
	EAfter: Into<E>
{
	Delimited {
		parser_before,
		parser,
		parser_after,
		__marker: PhantomData
	}
}

#[inline]
pub fn take<N>(amount: N) -> Take
where
	N: IntoUsize
{
	Take { inner: stateless::take(amount) }
}

#[inline]
pub fn take_const<const N: usize>() -> TakeConst<N> {
	TakeConst { inner: stateless::take_const() }
}

#[inline]
pub fn void<P, D, O, E>(parser: P) -> Void<P, D, O, E>
where
	P: ParserStateful<D, O, E>
{
	Void { parser, __marker: PhantomData }
}

#[expect(
	clippy::type_complexity,
	reason = "good naming makes it look alright I guess lol"
)]
pub struct Delimited<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter> {
	parser_before: PBefore,
	parser: P,
	parser_after: PAfter,
	__marker: PhantomData<(
		fn(D) -> (O, E),
		fn(D) -> (OBefore, EBefore),
		fn(D) -> (OAfter, EAfter)
	)>
}

impl<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter> ParserStateful<D, O, E>
for Delimited<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	P: ParserStateful<D, O, E>,
	PBefore: ParserStateful<D, OBefore, EBefore>,
	PAfter: ParserStateful<D, OAfter, EAfter>,
	EBefore: Into<E>,
	EAfter: Into<E>
{
	#[inline]
	fn parse_stateful(&mut self, data: D) -> Result<D, O, E> {
		let Success {
			output: _output_before,
			data
		} = self.parser_before.parse_stateful(data).map_err(Error::into)?;

		let Success { output, data } = self.parser.parse_stateful(data)?;

		let Success {
			output: _output_after,
			data
		} = self.parser_after.parse_stateful(data).map_err(Error::into)?;

		Ok(Success { output, data })
	}
}

pub struct Take {
	inner: stateless::Take
}

impl<'h> ParserStateful<&'h [u8], &'h [u8]> for Take {
	#[inline]
	fn parse_stateful(&mut self, data: &'h [u8]) -> Result<&'h [u8], &'h [u8]> {
		self.inner.parse(data)
	}
}

pub struct TakeConst<const N: usize> {
	inner: stateless::TakeConst<N>
}

impl<'h, const N: usize> ParserStateful<&'h [u8], &'h [u8; N]> for TakeConst<N> {
	#[inline]
	fn parse_stateful(&mut self, data: &'h [u8]) -> Result<&'h [u8], &'h [u8; N]> {
		self.inner.parse(data)
	}
}

pub struct Void<P, D, O, E> {
	parser: P,
	__marker: PhantomData<fn(D) -> (O, E)>
}

impl<P, D, O, E> ParserStateful<D, (), E> for Void<P, D, O, E>
where
	P: ParserStateful<D, O, E>
{
	#[inline]
	fn parse_stateful(&mut self, data: D) -> Result<D, (), E> {
		let Success { output: _void, data } = self.parser.parse_stateful(data)?;
		Ok(Success { output: (), data })
	}
}
