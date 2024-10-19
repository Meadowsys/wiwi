use crate::prelude_std::*;
use super::chain_fn;

super::decl_chain! {
	/// test uwu
	generics_decl: [T]
	generics: [T]
	chain: VecChain
	inner: Vec<T>
}

impl<T> VecChain<T> {
	chain_fn! {
		binary_search(inner, x: &T, out: &mut Result<usize, usize>) where {
			T: Ord
		} => *out = inner.binary_search(x)
	}

	chain_fn! {
		/// # Safety
		///
		/// `new_len` must be less than or equal to `capacity`, and
		/// the first `new_len` elements must be initialised/
		unsafe set_len(inner, new_len: usize)
			// SAFETY: caller promises that `new_len <= capacity` and
			// `..new_len` elements are initialised
			=> unsafe { inner.set_len(new_len) }
	}
}
