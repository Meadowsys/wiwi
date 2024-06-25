pub mod old;
pub use old::*;

mod traits;

mod array;
mod slice_box;
mod slice_mut;
mod slice_ref;
mod vec;

/// Temporary
pub mod new {
	pub use super::traits::{ ChainHalf, NonChainHalf };
	pub use super::vec::{ vec_chain, VecChain };
}

// TODO: check callbacks and use chaining apis there? maybe?

macro_rules! chainer {
	{
		$(#[$meta:meta])*
		$(
			generics_decl: [$($generics_decl:tt)*]
			generics: [$($generics:tt)*]
		)?
		chainer: $chainer:ident
		nonchain: $($nonchain:tt)+
	} => {
		$(#[$meta])*
		#[repr(transparent)]
		#[must_use = "chainers always takes ownership of itself, performs the operation, then returns itself again"]
		pub struct $chainer$(<$($generics_decl)*>)? {
			inner: $($nonchain)+
		}

		impl$(<$($generics_decl)*>)? $crate::chainer::traits::private::Sealed for $chainer$(<$($generics)*>)? {}
		impl$(<$($generics_decl)*>)? $crate::chainer::traits::private::Sealed for $($nonchain)+ {}

		impl$(<$($generics_decl)*>)? $crate::chainer::traits::NonChainHalf for $($nonchain)+ {
			type Chain = $chainer$(<$($generics)*>)?;
		}

		impl$(<$($generics_decl)*>)? $crate::chainer::traits::ChainHalf for $chainer$(<$($generics)*>)? {
			type NonChain = $($nonchain)+;
		}

		impl$(<$($generics_decl)*>)? ::std::convert::From<$($nonchain)+> for $chainer$(<$($generics)*>)? {
			#[inline]
			fn from(nonchain: $($nonchain)+) -> Self {
				Self { inner: nonchain }
			}
		}

		impl$(<$($generics_decl)*>)? ::std::convert::From<$chainer$(<$($generics)*>)?> for $($nonchain)+ {
			#[inline]
			fn from(chainer: $chainer$(<$($generics)*>)?) -> Self {
				chainer.inner
			}
		}

		impl$(<$($generics_decl)*>)? ::std::convert::AsRef<$($nonchain)+> for $chainer$(<$($generics)*>)? {
			#[inline]
			fn as_ref(&self) -> &$($nonchain)+ {
				&self.inner
			}
		}

		impl$(<$($generics_decl)*>)? ::std::convert::AsMut<$($nonchain)+> for $chainer$(<$($generics)*>)? {
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

			$self
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

			self
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

			$self
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

			self
		}
	};
}
use chain_fn;
