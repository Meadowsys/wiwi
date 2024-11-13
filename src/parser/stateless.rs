use crate::prelude_std::*;
use crate::num::*;
use super::{ Error, Result, Success };
use std::num::NonZero;

pub trait Parser<D, O, E = ()> {
	fn parse(&self, data: D) -> Result<D, O, E>;
}

#[inline]
pub fn delimited<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>(
	parser_before: PBefore,
	parser: P,
	parser_after: PAfter
) -> Delimited<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	P: Parser<D, O, E>,
	PBefore: Parser<D, OBefore, EBefore>,
	PAfter: Parser<D, OAfter, EAfter>,
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
	Take { amount: amount.into_usize() }
}

#[inline]
pub fn take_const<const N: usize>() -> TakeConst<N> {
	TakeConst { __private: () }
}

#[inline]
pub fn void<P, D, O, E>(parser: P) -> Void<P, D, O, E>
where
	P: Parser<D, O, E>
{
	Void { parser, __marker: PhantomData }
}

#[expect(
	clippy::type_complexity,
	reason = "good naming makes it look alright I guess lol"
)]
pub struct Delimited<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	P: Parser<D, O, E>,
	PBefore: Parser<D, OBefore, EBefore>,
	PAfter: Parser<D, OAfter, EAfter>,
	EBefore: Into<E>,
	EAfter: Into<E>
{
	parser_before: PBefore,
	parser: P,
	parser_after: PAfter,
	__marker: PhantomData<(
		fn(D) -> (O, E),
		fn(D) -> (OBefore, EBefore),
		fn(D) -> (OAfter, EAfter)
	)>
}

impl<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter> Parser<D, O, E>
for Delimited<P, D, O, E, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	P: Parser<D, O, E>,
	PBefore: Parser<D, OBefore, EBefore>,
	PAfter: Parser<D, OAfter, EAfter>,
	EBefore: Into<E>,
	EAfter: Into<E>
{
	#[inline]
	fn parse(&self, data: D) -> Result<D, O, E> {
		let Success {
			output: _output_before,
			data
		} = self.parser_before.parse(data).map_err(Error::into)?;

		let Success { output, data } = self.parser.parse(data)?;

		let Success {
			output: _output_after,
			data
		} = self.parser_after.parse(data).map_err(Error::into)?;

		Ok(Success { output, data })
	}
}

pub struct Take {
	amount: usize
}

impl<'h> Parser<&'h [u8], &'h [u8]> for Take {
	#[inline]
	fn parse(&self, data: &'h [u8]) -> Result<&'h [u8], &'h [u8]> {
		data.split_at_checked(self.amount)
			.map(|(output, data)| Success { output, data })
			.ok_or_else(|| Error::NotEnoughData {
				missing: NonZero::new(self.amount - data.len())
			})
	}
}

pub struct TakeConst<const N: usize> {
	__private: ()
}

impl<'h, const N: usize> Parser<&'h [u8], &'h [u8; N]> for TakeConst<N> {
	#[inline]
	fn parse(&self, data: &'h [u8]) -> Result<&'h [u8], &'h [u8; N]> {
		data.split_at_checked(N)
			.map(|(output, data)| Success {
				// SAFETY: output is going to be N length because `split_at_checked`
				output: unsafe {
					#[expect(clippy::as_conversions, reason = "ptr cast")]
					&*(output as *const [u8] as *const [u8; N])
				},
				data
			})
			.ok_or_else(|| Error::NotEnoughData {
				missing: NonZero::new(N - data.len())
			})
	}
}

pub struct Void<P, D, O, E>
where
	P: Parser<D, O, E>
{
	parser: P,
	/// ??? lol
	__marker: PhantomData<fn(D) -> (O, E)>
}

impl<P, D, O, E> Parser<D, (), E> for Void<P, D, O, E>
where
	P: Parser<D, O, E>
{
	#[inline]
	fn parse(&self, data: D) -> Result<D, (), E> {
		let Success { output: _void, data } = self.parser.parse(data)?;
		Ok(Success { output: (), data })
	}
}
