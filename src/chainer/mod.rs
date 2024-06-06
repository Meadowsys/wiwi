pub mod old;

mod traits;
pub use traits::{ ChainHalf, NonChainHalf };

mod vec;
pub use vec::VecChain;

macro_rules! chainer {
	{
		$(#[$meta:meta])*
		chainer: $chain:ident$(<$($generics:ident $(= $generic_default:path)?),*>)?
		inner: $nonchain:path
	} => {
		$(#[$meta])*
		#[repr(transparent)]
		#[must_use = "chainers always takes ownership of itself, performs the operation, then returns itself again"]
		pub struct $chain$(<$($generics $(= $generic_default)?),*>)? {
			inner: $nonchain
		}

		impl$(<$($generics),*>)? $crate::chainer::traits::private::Sealed for $chain$(<$($generics),*>)? {}
		impl$(<$($generics),*>)? $crate::chainer::traits::private::Sealed for $nonchain {}

		impl$(<$($generics),*>)? $crate::chainer::NonChainHalf for $nonchain {
			type Chain = $chain$(<$($generics),*>)?;
		}

		impl$(<$($generics),*>)? $crate::chainer::ChainHalf for $chain$(<$($generics),*>)? {
			type NonChain = $nonchain;
		}

		impl$(<$($generics),*>)? ::std::convert::From<$nonchain> for $chain$(<$($generics),*>)? {
			#[inline]
			fn from(nonchain: $nonchain) -> Self {
				Self { inner: nonchain }
			}
		}

		impl$(<$($generics),*>)? ::std::convert::From<$chain$(<$($generics),*>)?> for $nonchain {
			#[inline]
			fn from(chainer: $chain$(<$($generics),*>)?) -> Self {
				chainer.inner
			}
		}

		impl$(<$($generics),*>)? ::std::convert::AsRef<$nonchain> for $chain$(<$($generics),*>)? {
			#[inline]
			fn as_ref(&self) -> &$nonchain {
				&self.inner
			}
		}

		impl$(<$($generics),*>)? ::std::convert::AsMut<$nonchain> for $chain$(<$($generics),*>)? {
			#[inline]
			fn as_mut(&mut self) -> &mut $nonchain {
				&mut self.inner
			}
		}
	}
}
use chainer;

macro_rules! chain_fn {
	// too many duplicate code I don't like this aa

	// unsafe self
	{
		$(#[$meta:meta])*
		unsafe self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			{ $body }

			// let it handle things like todo! macro etc
			#[allow(unreachable_code)]
			{ $self }
		}
	};

	// unsafe
	{
		$(#[$meta:meta])*
		unsafe $fn_name:ident$([$($generics:tt)*])?($inner:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			use $crate::chainer::ChainHalf as _;

			let $inner = self.as_nonchain_mut();
			{ $body }

			// let it handle things like todo! macro etc
			#[allow(unreachable_code)]
			{ self }
		}
	};

	// takes self
	{
		$(#[$meta:meta])*
		self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			{ $body }

			// let it handle things like todo! macro etc
			#[allow(unreachable_code)]
			{ $self }
		}
	};

	// regular
	{
		$(#[$meta:meta])*
		$fn_name:ident$([$($generics:tt)*])?($inner:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			use $crate::chainer::ChainHalf as _;

			let $inner = self.as_nonchain_mut();
			{ $body }

			// let it handle things like todo! macro etc
			#[allow(unreachable_code)]
			{ self }
		}
	};
}
use chain_fn;
