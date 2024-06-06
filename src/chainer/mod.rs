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
			fn from(nonchain: $nonchain) -> Self {
				Self { inner: nonchain }
			}
		}

		impl$(<$($generics),*>)? ::std::convert::From<$chain$(<$($generics),*>)?> for $nonchain {
			fn from(chainer: $chain$(<$($generics),*>)?) -> Self {
				chainer.inner
			}
		}
	}
}
use chainer;

macro_rules! chain_fn {
	($fn_name:ident($inner:ident $(, $($args:tt)*)?) => $body:expr) => {
		#[inline]
		pub fn $fn_name(self $(, $($args)*)?) -> Self {
			use $crate::chainer::{ ChainHalf as _, NonChainHalf as _ };

			let mut $inner = self.into_nonchain();
			{ $body }
			$inner.into_chainer()
		}
	}
}

use chain_fn;
macro_rules! chain_fn_self {
	($fn_name:ident($self:ident $(, $($args:tt)*)?) => $body:expr) => {
		#[inline]
		pub fn $fn_name(mut $self $(, $($args)*)?) -> Self {
			{ $body }
			$self
		}
	}
}
use chain_fn_self;
