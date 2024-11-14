use crate::prelude_std::*;
use crate::num::*;
use super::{ Error, Input, Needle, ParserPhantom, Result, Success };
use std::num::NonZero;

pub trait Parser<I, O, E = ()>
where
	I: Input
{
	fn parse(&self, data: I) -> Result<I, O, E>;
}

impl<F, I, O, E> Parser<I, O, E> for F
where
	I: Input,
	F: Fn(I) -> Result<I, O, E>
{
	#[inline]
	fn parse(&self, data: I) -> Result<I, O, E> {
		self(data)
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
	PBefore: Parser<I, OBefore, EBefore>,
	P: Parser<I, O, E>,
	PAfter: Parser<I, OAfter, EAfter>,
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
	Tag { tag }
}

#[inline]
pub fn take<N>(amount: N) -> Take
where
	N: IntoUsize
{
	Take { amount: amount.into_usize() }
}

#[inline]
pub fn take_const<const N: usize>() -> TakeConst<N> {
	TakeConst { __private: () }
}

#[inline]
pub fn void<P, I, O, E>(parser: P) -> Void<P, I, O, E>
where
	I: Input,
	P: Parser<I, O, E>
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

impl<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter> Parser<I, O, EReal>
for Delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	I: Input,
	PBefore: Parser<I, OBefore, EBefore>,
	P: Parser<I, O, E>,
	PAfter: Parser<I, OAfter, EAfter>,
	EBefore: Into<EReal>,
	E: Into<EReal>,
	EAfter: Into<EReal>
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, O, EReal> {
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
	tag: T
}

impl<I, T> Parser<I, ()> for Tag<T>
where
	I: Input,
	T: Needle<I>
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, ()> {
		let true = input.len() >= self.tag.len() else {
			return Err(Error::NotEnoughData {
				missing: NonZero::new(self.tag.len() - input.len())
			})
		};

		self.tag.input_starts_with(&input)
			.then(|| input.take_first(self.tag.len()))
			.flatten()
			.map(|(_, remaining_input)| Success { output: (), remaining_input })
			.ok_or_else(|| Error::Error { error: () })
	}
}

#[repr(transparent)]
pub struct Take {
	amount: usize
}

impl<I> Parser<I, I> for Take
where
	I: Input
{
	#[inline]
	fn parse(&self, data: I) -> Result<I, I> {
		data.take_first(self.amount)
			.map(|(output, remaining_input)| Success { output, remaining_input })
			.ok_or_else(|| Error::NotEnoughData {
				missing: NonZero::new(self.amount - data.len())
			})
	}
}

#[repr(transparent)]
pub struct TakeConst<const N: usize> {
	__private: ()
}

impl<I, const N: usize> Parser<I, I::ConstSize<N>> for TakeConst<N>
where
	I: Input
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, I::ConstSize<N>> {
		input.take_first_const()
			.map(|(output, remaining_input)| Success { output, remaining_input })
			.ok_or_else(|| Error::NotEnoughData {
				missing: NonZero::new(N - input.len())
			})
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

impl<P, I, O, E> Parser<I, (), E> for Void<P, I, O, E>
where
	I: Input,
	P: Parser<I, O, E>
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, (), E> {
		let Success { output: _void, remaining_input } = self.parser.parse(input)?;
		Ok(Success { output: (), remaining_input })
	}
}
