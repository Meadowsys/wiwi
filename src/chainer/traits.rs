use super::*;

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
pub trait NonChainHalf: Sized + private::Sealed
where
	Self: Into<Self::Chain>,
	Self::Chain: Into<Self>
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

// /// Convert any chain type back into its original, non-chaining-API type
// ///
// /// This is the reversal operation of [`NonChainHalf`].
pub trait ChainHalf: Sized + private::Sealed
where
	Self: Into<Self::NonChain>,
	Self::NonChain: Into<Self>
{
	type NonChain;

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

pub(super) mod private {
	pub trait Sealed {}
}
