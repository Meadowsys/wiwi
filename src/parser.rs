use crate::prelude_std::*;
use crate::num::*;
use std::num::NonZero;

pub trait Parser<D, O, E = ()> {
	fn parse(&mut self, data: D) -> Result<D, O, E>;
}

pub struct Success<D, O> {
	pub output: O,
	pub data: D,
}

pub enum Error<E> {
	NotEnoughData { missing: Option<NonZero<usize>> },
	Error { error: E },
	Fatal { error: E }
}

pub type Result<D, O, E = ()> = std::result::Result<Success<D, O>, Error<E>>;

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

pub struct Take {
	amount: usize
}

impl<'h> Parser<&'h [u8], &'h [u8]> for Take {
	#[inline]
	fn parse(&mut self, data: &'h [u8]) -> Result<&'h [u8], &'h [u8]> {
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
	fn parse(&mut self, data: &'h [u8]) -> Result<&'h [u8], &'h [u8; N]> {
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
	fn parse(&mut self, data: D) -> Result<D, (), E> {
		let Success { output: _void, data } = self.parser.parse(data)?;
		Ok(Success { output: (), data })
	}
}
