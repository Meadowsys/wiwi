pub mod old;
pub use old::*;

mod traits;
mod vec;

/// Temporary
pub mod new {
	pub use super::traits::{ ChainHalf, NonChainHalf };
	pub use super::vec::{ vec_chain, VecChain };
}

macro_rules! chainer {
	{
		$(#[$meta:meta])*
		chainer: $chain:ident$(<$($generics:tt $(= $generic_default:path)?),*>)?
		inner: $($nonchain:tt)+
	} => {
		$(#[$meta])*
		#[repr(transparent)]
		#[must_use = "chainers always takes ownership of itself, performs the operation, then returns itself again"]
		pub struct $chain$(<$($generics $(= $generic_default)?),*>)? {
			inner: $($nonchain)+
		}

		impl$(<$($generics),*>)? $crate::chainer::traits::private::Sealed for $chain$(<$($generics),*>)? {}
		impl$(<$($generics),*>)? $crate::chainer::traits::private::Sealed for $($nonchain)+ {}

		impl$(<$($generics),*>)? $crate::chainer::traits::NonChainHalf for $($nonchain)+ {
			type Chain = $chain$(<$($generics),*>)?;
		}

		impl$(<$($generics),*>)? $crate::chainer::traits::ChainHalf for $chain$(<$($generics),*>)? {
			type NonChain = $($nonchain)+;
		}

		impl$(<$($generics),*>)? ::std::convert::From<$($nonchain)+> for $chain$(<$($generics),*>)? {
			#[inline]
			fn from(nonchain: $($nonchain)+) -> Self {
				Self { inner: nonchain }
			}
		}

		impl$(<$($generics),*>)? ::std::convert::From<$chain$(<$($generics),*>)?> for $($nonchain)+ {
			#[inline]
			fn from(chainer: $chain$(<$($generics),*>)?) -> Self {
				chainer.inner
			}
		}

		impl$(<$($generics),*>)? ::std::convert::AsRef<$($nonchain)+> for $chain$(<$($generics),*>)? {
			#[inline]
			fn as_ref(&self) -> &$($nonchain)+ {
				&self.inner
			}
		}

		impl$(<$($generics),*>)? ::std::convert::AsMut<$($nonchain)+> for $chain$(<$($generics),*>)? {
			#[inline]
			fn as_mut(&mut self) -> &mut $($nonchain)+ {
				&mut self.inner
			}
		}
	}
}
use chainer;

macro_rules! chain_fn {
	// too many duplicate code I don't like this aa

	{
		$(#[$meta:meta])*
		unsafe move self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			$body
		}
	};

	{
		$(#[$meta:meta])*
		unsafe move $fn_name:ident$([$($generics:tt)*])?($inner:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			use $crate::chainer::traits::{ ChainHalf as _, NonChainHalf as _ };

			let mut $inner = self.into_nonchain();
			$body.into_chainer()
		}
	};

	{
		$(#[$meta:meta])*
		move self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			$body
		}
	};

	{
		$(#[$meta:meta])*
		move $fn_name:ident$([$($generics:tt)*])?($inner:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			use $crate::chainer::traits::{ ChainHalf as _, NonChainHalf as _ };

			let $inner = self.into_nonchain();
			$body.into_chainer()
		}
	};

	{
		$(#[$meta:meta])*
		unsafe self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			$body;

			// shush warning for things like todo! macro etc
			#[allow(unreachable_code)]
			{ $self }
		}
	};

	{
		$(#[$meta:meta])*
		unsafe $fn_name:ident$([$($generics:tt)*])?($inner:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			use $crate::chainer::traits::ChainHalf as _;

			let $inner = self.as_nonchain_mut();
			$body;

			// shush warning for things like todo! macro etc
			#[allow(unreachable_code)]
			{ self }
		}
	};

	{
		$(#[$meta:meta])*
		self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			$body;

			// shush warning for things like todo! macro etc
			#[allow(unreachable_code)]
			{ $self }
		}
	};

	{
		$(#[$meta:meta])*
		$fn_name:ident$([$($generics:tt)*])?($inner:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			use $crate::chainer::traits::ChainHalf as _;

			let $inner = self.as_nonchain_mut();
			$body;

			// shush warning for things like todo! macro etc
			#[allow(unreachable_code)]
			{ self }
		}
	};
}
use chain_fn;
