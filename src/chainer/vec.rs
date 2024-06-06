use super::{ chainer, chain_fn, chain_fn_self, ChainHalf, NonChainHalf };

chainer! {
	chainer: VecChain<T>
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

	chain_fn!(test(inner, count: &mut usize) => *count = inner.len());
	chain_fn_self!(test2(self, count: &mut usize) => *count = self.inner.len());
}

chainer! {
	chainer: HashMapChain<K, V, S = std::hash::RandomState>
	inner: std::collections::HashMap<K, V, S>
}
