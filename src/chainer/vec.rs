use super::{ chainer, chain_fn, ChainHalf, NonChainHalf };

chainer! {
	chainer: VecChain[T]
	inner: Vec<T>
}

impl<T> VecChain<T> {
	pub fn newnenwne(thing: Vec<T>) -> Self {
		thing.into_chainer()
	}
	pub fn newneeenwne(thing: Vec<T>) -> Self {
		Self { inner: thing }
	}

	pub fn oiwefaowafeiijoafew(self) -> Vec<T> {
		self.into_nonchain()
	}
	pub fn oiwefaowafeiijoaefew(self) -> Vec<T> {
		self.inner
	}

	chain_fn!(test(this, count: &mut usize) => *count = this.inner.len());
}
