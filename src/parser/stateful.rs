use crate::prelude_std::*;
use crate::function::*;
use crate::num::*;
use super::{ stateless, util, Error, Input, Needle, Parser as _, ParserPhantom, Result, Success };

pub trait ParserStateful<I, O, E = ()>
where
	I: Input
{
	fn parse(&mut self, input: I) -> Result<I, O, E>;

	#[inline]
	fn map<F, O2>(self, f: F) -> util::Map<Self, F, O>
	where
		Self: Sized,
		F: FunctionMut<(O,), O2>
	{
		util::map(self, f)
	}
}

impl<F, I, O, E> ParserStateful<I, O, E> for F
where
	I: Input,
	F: FunctionMut<(I,), Result<I, O, E>>
{
	#[inline]
	fn parse(&mut self, data: I) -> Result<I, O, E> {
		self.call_mut((data,))
	}
}

#[inline]
pub fn delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>(
	parser_before: PBefore,
	parser: P,
	parser_after: PAfter
) -> Delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	I: Input,
	PBefore: ParserStateful<I, OBefore, EBefore>,
	P: ParserStateful<I, O, E>,
	PAfter: ParserStateful<I, OAfter, EAfter>,
	EBefore: Into<EReal>,
	E: Into<EReal>,
	EAfter: Into<EReal>
{
	Delimited {
		parser_before,
		parser,
		parser_after,
		__marker: PhantomData
	}
}

#[inline]
pub fn tag<T>(tag: T) -> Tag<T> {
	Tag { inner: stateless::tag(tag) }
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
pub fn void<P, I, O, E>(parser: P) -> Void<P, I, O, E>
where
	I: Input,
	P: ParserStateful<I, O, E>
{
	Void { parser, __marker: PhantomData }
}

#[expect(
	clippy::type_complexity,
	reason = "good naming makes it look alright I guess lol"
)]
pub struct Delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	I: Input
{
	parser_before: PBefore,
	parser: P,
	parser_after: PAfter,
	__marker: PhantomData<(
		ParserPhantom<I, OBefore, EBefore>,
		ParserPhantom<I, O, E>,
		ParserPhantom<I, OAfter, EAfter>,
		fn() -> EReal
	)>
}

impl<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter> ParserStateful<I, O, EReal>
for Delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	I: Input,
	PBefore: ParserStateful<I, OBefore, EBefore>,
	P: ParserStateful<I, O, E>,
	PAfter: ParserStateful<I, OAfter, EAfter>,
	EBefore: Into<EReal>,
	E: Into<EReal>,
	EAfter: Into<EReal>
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, O, EReal> {
		let Success {
			output: _output_before,
			remaining_input: input
		} = self.parser_before.parse(input).map_err(Error::into)?;

		let Success {
			output,
			remaining_input: input
		} = self.parser.parse(input).map_err(Error::into)?;

		let Success {
			output: _output_after,
			remaining_input
		} = self.parser_after.parse(input).map_err(Error::into)?;

		Ok(Success { output, remaining_input })
	}
}

#[repr(transparent)]
pub struct Tag<T> {
	inner: stateless::Tag<T>
}

impl<I, T> ParserStateful<I, ()> for Tag<T>
where
	I: Input,
	T: Needle<I>
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, ()> {
		self.inner.parse(input)
	}
}

#[repr(transparent)]
pub struct Take {
	inner: stateless::Take
}

impl<I> ParserStateful<I, I> for Take
where
	I: Input
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, I> {
		self.inner.parse(input)
	}
}

#[repr(transparent)]
pub struct TakeConst<const N: usize> {
	inner: stateless::TakeConst<N>
}

impl<I, const N: usize> ParserStateful<I, I::ConstSize<N>> for TakeConst<N>
where
	I: Input
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, I::ConstSize<N>> {
		self.inner.parse(input)
	}
}

#[repr(transparent)]
pub struct Void<P, I, O, E>
where
	I: Input
{
	parser: P,
	__marker: ParserPhantom<I, O, E>
}

impl<P, I, O, E> ParserStateful<I, (), E> for Void<P, I, O, E>
where
	I: Input,
	P: ParserStateful<I, O, E>
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, (), E> {
		let Success { output: _void, remaining_input } = self.parser.parse(input)?;
		Ok(Success { output: (), remaining_input })
	}
}
