use crate::prelude_std::*;

mod vec;
pub use vec::VecChain;

pub trait Chain
where
	Self: Sized + private::Sealed + Into<Self::Inner> + AsRef<Self::Inner> + AsMut<Self::Inner>,
	Self::Inner: Sized + ChainInner<Chain = Self>
{
	type Inner;

	#[inline]
	fn into_inner(self) -> Self::Inner {
		self.into()
	}

	#[inline]
	fn from_inner(inner: Self::Inner) -> Self {
		inner.into()
	}

	#[inline]
	fn as_inner(&self) -> &Self::Inner {
		self.as_ref()
	}

	#[inline]
	fn as_inner_mut(&mut self) -> &mut Self::Inner {
		self.as_mut()
	}

	#[inline]
	fn with_inner<F>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut Self)
	{
		f(&mut self);
		self
	}
}

pub trait ChainInner
where
	Self: Sized + private::Sealed + Into<Self::Chain>,
	Self::Chain: Sized + Chain<Inner = Self>
{
	type Chain;

	#[inline]
	fn into_chain(self) -> Self::Chain {
		self.into()
	}

	#[inline]
	fn from_chain(chain: Self::Chain) -> Self {
		chain.into()
	}
}

macro_rules! decl_chain {
	{
		$(#[$meta:meta])*
		$(
			generics_decl: [$($generics_decl:tt)*]
			generics: [$($generics:tt)*]
		)?
		chain: $chain:ident
		inner: $($inner:tt)+
	} => {
		// the struct declaration
		$(#[$meta])*
		#[must_use = "a chain always takes ownership of itself, performs the operation, then returns itself again"]
		pub struct $chain$(<$($generics_decl)*>)? {
			_inner: $($inner)+
		}

		// the private::Sealed impls
		impl$(<$($generics_decl)*>)? $crate::chain::private::Sealed for $chain$(<$($generics)*>)? {}
		impl$(<$($generics_decl)*>)? $crate::chain::private::Sealed for $($inner)* {}

		// impl Chain
		impl$(<$($generics_decl)*>)? $crate::chain::Chain for $chain$(<$($generics)*>)? {
			type Inner = $($inner)+;
		}

		// impl ChainInner
		impl$(<$($generics_decl)*>)? $crate::chain::ChainInner for $($inner)* {
			type Chain = $chain$(<$($generics)*>)?;
		}

		// impl From<Inner> for Chain
		impl$(<$($generics_decl)*>)? $crate::prelude_std::From<$($inner)+> for $chain$(<$($generics)*>)? {
			#[inline]
			fn from(inner: $($inner)+) -> Self {
				Self { _inner: inner }
			}
		}

		// impl From<Chain> for Inner
		impl$(<$($generics_decl)*>)? $crate::prelude_std::From<$chain$(<$($generics)*>)?> for $($inner)+ {
			#[inline]
			fn from(chain: $chain$(<$($generics)*>)?) -> Self {
				chain._inner
			}
		}

		// impl AsRef<Inner>
		impl$(<$($generics_decl)*>)? $crate::prelude_std::AsRef<$($inner)+> for $chain$(<$($generics)*>)? {
			#[inline]
			fn as_ref(&self) -> &$($inner)+ {
				&self._inner
			}
		}

		// impl AsMut<Inner>
		impl$(<$($generics_decl)*>)? $crate::prelude_std::AsMut<$($inner)+> for $chain$(<$($generics)*>)? {
			#[inline]
			fn as_mut(&mut self) -> &mut $($inner)+ {
				&mut self._inner
			}
		}

		// impl Clone
		// impl Copy
		// impl Debug
		// impl Default
		// impl Display
		// impl Eq
		// impl Ord
		// impl PartialEq/PartialOrd chain <-> chain
		// impl PartialEq/PartialOrd chain <-> inner
		// impl PartialEq/PartialOrd inner <-> chain
	};
}
use decl_chain;

macro_rules! chain_fn {
	{
		$(#[$meta:meta])*
		unsafe move self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			$body
		}
	};

	{
		$(#[$meta:meta])*
		unsafe move
		$fn_name:ident
		$([$($generics:tt)*])?
		($nc:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let mut $nc = <Self as $crate::chain::Chain>::into_inner(self);
			<Self as $crate::chain::Chain>::from_inner($body)
		}
	};

	{
		$(#[$meta:meta])*
		move self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			$body
		}
	};

	{
		$(#[$meta:meta])*
		move
		$fn_name:ident
		$([$($generics:tt)*])?
		($nc:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let mut $nc = <Self as $crate::chain::Chain>::into_inner(self);
			<Self as $crate::chain::Chain>::from_inner($body)
		}
	};

	{
		$(#[$meta:meta])*
		unsafe self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> void $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let _ = $body;
			$self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe
		$fn_name:ident
		$([$($generics:tt)*])?
		($nc:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> void $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let $nc = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);
			let _ = $body;
			self
		}
	};

	{
		$(#[$meta:meta])*
		self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> void $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let _ = $body;
			$self
		}
	};

	{
		$(#[$meta:meta])*
		$fn_name:ident
		$([$($generics:tt)*])?
		($nc:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> void $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let $nc = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);
			let _ = $body;
			self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			$crate::prelude_std::identity::<()>($body);
			$self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe
		$fn_name:ident
		$([$($generics:tt)*])?
		($nc:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let $nc = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);
			$crate::prelude_std::identity::<()>($body);
			self
		}
	};

	{
		$(#[$meta:meta])*
		self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			$crate::prelude_std::identity::<()>($body);
			$self
		}
	};

	{
		$(#[$meta:meta])*
		$fn_name:ident
		$([$($generics:tt)*])?
		($nc:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let $nc = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);
			$crate::prelude_std::identity::<()>($body);
			self
		}
	};
}
use chain_fn;

/// notouchie
mod private {
	/// notouchie
	pub trait Sealed {}
}
