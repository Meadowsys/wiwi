use super::*;
use std::convert::{ AsRef, AsMut };

// /// Convert any chain type back into its original, non-chaining-API type
// ///
// /// This is the reversal operation of [`NonChainHalf`].
pub trait ChainHalf
where
	Self: Sized + private::Sealed + Into<Self::NonChain> + AsRef<Self::NonChain> + AsMut<Self::NonChain>,
	Self::NonChain: Into<Self> + NonChainHalf<Chain = Self>
{
	type NonChain;

	#[inline]
	fn as_nonchain(&self) -> &Self::NonChain {
		self.as_ref()
	}

	#[inline]
	fn as_nonchain_mut(&mut self) -> &mut Self::NonChain {
		self.as_mut()
	}

	// /// Converts `self` into the original, non-chaining-API type
	#[inline]
	fn into_nonchain(self) -> Self::NonChain {
		self.into()
	}

	#[inline]
	fn from_nonchain(nonchain: Self::NonChain) -> Self {
		nonchain.into()
	}
}

// /// Convert any supported type into its chainer type
// ///
// /// Every type that implements this trait has a "preferred" chain type, declared
// /// by the [`NonChainHalf::Chain`] associated type, and calling [`into_chainer`]
// /// will convert it to that chain type.
// ///
// /// To undo this operation, you can use [`ChainHalf`].
// ///
// /// [`into_chainer`]: NonChainHalf::into_chainer
// ///
// /// # Examples
// ///
// /// ```
// /// # use wiwi::chainer::{ NonChainHalf, VecChain };
// /// let vec: Vec<String> = Vec::new();
// /// let chain: VecChain<String> = vec.into_chainer();
// /// // ...
// /// // do funny things with chaining API c:
// /// ```
pub trait NonChainHalf
where
	Self: Sized + private::Sealed + Into<Self::Chain>,
	Self::Chain: Into<Self> + ChainHalf<NonChain = Self> + AsRef<Self> + AsMut<Self>
{
	// /// The preferred chainer of this type
	type Chain;

	// /// Converts `self` into its associated chain type
	#[inline]
	fn into_chainer(self) -> Self::Chain {
		self.into()
	}

	#[inline]
	fn from_chainer(chainer: Self::Chain) -> Self {
		chainer.into()
	}
}

pub(super) mod private {
	pub trait Sealed {}
}
