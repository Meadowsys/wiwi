pub mod old;

mod traits;
pub use traits::{ ChainHalf, NonChainHalf };

mod vec;
pub use vec::VecChain;

macro_rules! chainer {
	{
		$(#[$meta:meta])*
		chainer: $chain:ident$([$($generics:tt)*])?
		inner: $nonchain:path
	} => {
		$(#[$meta])*
		#[repr(transparent)]
		#[must_use = "chainers always takes ownership of itself, performs the operation, then returns itself again"]
		pub struct $chain$(<$($generics)*>)? {
			inner: $nonchain
		}

		impl$(<$($generics)*>)? $crate::chainer::traits::private::Sealed for $chain$(<$($generics)*>)? {}
		impl$(<$($generics)*>)? $crate::chainer::traits::private::Sealed for $nonchain {}

		impl$(<$($generics)*>)? $crate::chainer::NonChainHalf for $nonchain {
			type Chain = $chain$(<$($generics)*>)?;
		}

		impl$(<$($generics)*>)? $crate::chainer::ChainHalf for $chain$(<$($generics)*>)? {
			type NonChain = $nonchain;
		}

		impl$(<$($generics)*>)? From<$nonchain> for $chain$(<$($generics)*>)? {
			fn from(nonchain: $nonchain) -> Self {
				Self { inner: nonchain }
			}
		}

		impl$(<$($generics)*>)? From<$chain$(<$($generics)*>)?> for $nonchain {
			fn from(chainer: $chain$(<$($generics)*>)?) -> Self {
				chainer.inner
			}
		}
	}
}
use chainer;

macro_rules! chain_fn {
	($fn_name:ident($self:ident$(, $($args:tt)*)?) => $body:expr) => {
		pub fn $fn_name(self $(, $($args)*)?) -> Self {
			#[allow(unused_mut)]
			let mut $self = self;
			{ $body }
			$self
		}
	}
}
use chain_fn;
