use crate::prelude_std::*;
use super::{ stateful, stateless, Input, ParserPhantom, Result };

/// Wraps an implementor of [`Parser`](stateless::Parser) and provides an implementation
/// of [`ParserStateful`](stateful::ParserStateful)
#[inline]
pub fn adapt_stateful<P, I, O, E>(parser: P) -> AdaptStateful<P, I, O, E>
where
	I: Input,
	P: stateless::Parser<I, O, E>
{
	AdaptStateful { parser, __marker: PhantomData }
}

#[repr(transparent)]
pub struct AdaptStateful<P, I, O, E>
where
	I: Input
{
	parser: P,
	__marker: ParserPhantom<I, O, E>
}

impl<P, I, O, E> stateful::ParserStateful<I, O, E> for AdaptStateful<P, I, O, E>
where
	I: Input,
	P: stateless::Parser<I, O, E>
{
	#[inline]
	fn parse_stateful(&mut self, input: I) -> Result<I, O, E> {
		self.parser.parse(input)
	}
}
