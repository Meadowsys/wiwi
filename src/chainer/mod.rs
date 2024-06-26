mod traits;
pub use traits::{ ChainHalf, NonChainHalf };

mod array;
pub use array::ArrayChain;
mod slice_box;
pub use slice_box::SliceBoxChain;
mod slice_mut;
pub use slice_mut::SliceMutChain;
mod slice_ref;
pub use slice_ref::SliceRefChain;
mod vec;
pub use vec::{ vec_chain, VecChain };

// TODO: check callbacks and use chaining apis there? maybe?

macro_rules! chainer {
	(@doclink link: $doclink:literal, nonchain: $($nonchain:tt)+) => { $doclink };

	(@doclink nonchain: $($nonchain:tt)+) => { stringify!($($nonchain)+) };

	{
		$(#[$meta:meta])*
		$(doclink: $doclink:literal)?
		$(
			generics_decl: [$($generics_decl:tt)*]
			generics: [$($generics:tt)*]
		)?
		chainer: $chainer:ident
		nonchain: $($nonchain:tt)+
	} => {
		/// Struct providing a chaining API for
		#[doc = concat!("[`", stringify!($($nonchain)+), "`]")]
		#[doc = ""]
		// this attr is problematic, for some reason
		// #[doc = concat!("[`", stringify!($($nonchain)+), "`]: ", chainer!(@doclink $(link: $doclink,)? $($nonchain)+))]
		#[doc = ""]
		$(#[$meta])*
		#[repr(transparent)]
		#[must_use = "chainers always takes ownership of itself, performs the operation, then returns itself again"]
		pub struct $chainer$(<$($generics_decl)*>)? {
			_nc: $($nonchain)+
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
			#[inline(always)]
			fn from(nonchain: $($nonchain)+) -> Self {
				Self { _nc: nonchain }
			}
		}

		impl$(<$($generics_decl)*>)? ::std::convert::From<$chainer$(<$($generics)*>)?> for $($nonchain)+ {
			#[inline(always)]
			fn from(chainer: $chainer$(<$($generics)*>)?) -> Self {
				chainer._nc
			}
		}

		impl$(<$($generics_decl)*>)? ::std::convert::AsMut<$($nonchain)+> for $chainer$(<$($generics)*>)? {
			#[inline(always)]
			fn as_mut(&mut self) -> &mut $($nonchain)+ {
				&mut self._nc
			}
		}

		impl$(<$($generics_decl)*>)? ::std::convert::AsRef<$($nonchain)+> for $chainer$(<$($generics)*>)? {
			#[inline(always)]
			fn as_ref(&self) -> &$($nonchain)+ {
				&self._nc
			}
		}

		#[allow(clippy::non_canonical_clone_impl)] // shut
		impl$(<$($generics_decl)*>)? ::std::clone::Clone for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::clone::Clone
		{
			fn clone(&self) -> Self {
				<$($nonchain)+ as ::std::clone::Clone>::clone(&self.as_nonchain()).into()
			}
		}

		impl$(<$($generics_decl)*>)? ::std::marker::Copy for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::marker::Copy
		{}

		impl$(<$($generics_decl)*>)? ::std::fmt::Debug for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::fmt::Debug
		{
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				f.debug_struct(stringify!($chainer))
					.field("_", self.as_nonchain())
					.finish()
			}
		}

		impl$(<$($generics_decl)*>)? ::std::default::Default for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::default::Default
		{
			fn default() -> Self {
				<$($nonchain)+ as ::std::default::Default>::default().into()
			}
		}

		impl$(<$($generics_decl)*>)? ::std::fmt::Display for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::fmt::Display
		{
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				::std::fmt::Display::fmt(self.as_nonchain(), f)
			}
		}
	};
}
use chainer;

macro_rules! chain_fn {
	// too many duplicate code I don't like this aa

	{
		$(#[$meta:meta])*
		unsafe self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => void $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			let _ = $body;
			$self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe $fn_name:ident$([$($generics:tt)*])?($nc:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => void $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			let $nc = $crate::chainer::traits::ChainHalf::as_nonchain_mut(&mut self);
			let _ = $body;
			self
		}
	};

	{
		$(#[$meta:meta])*
		self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => void $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			let _ = $body;
			$self
		}
	};

	{
		$(#[$meta:meta])*
		$fn_name:ident$([$($generics:tt)*])?($nc:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => void $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			let $nc = $crate::chainer::traits::ChainHalf::as_nonchain_mut(&mut self);
			let _ = $body;
			self
		}
	};

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
		unsafe move $fn_name:ident$([$($generics:tt)*])?($nc:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			let mut $nc = $crate::chainer::traits::ChainHalf::into_nonchain(self);
			$crate::chainer::traits::NonChainHalf::into_chainer($body)
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
		move $fn_name:ident$([$($generics:tt)*])?($nc:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			let mut $nc = $crate::chainer::traits::ChainHalf::into_nonchain(self);
			$crate::chainer::traits::NonChainHalf::into_chainer($body)
		}
	};

	{
		$(#[$meta:meta])*
		unsafe self $fn_name:ident$([$($generics:tt)*])?($self:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			::std::convert::identity::<()>($body);
			$self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe $fn_name:ident$([$($generics:tt)*])?($nc:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			let $nc = $crate::chainer::traits::ChainHalf::as_nonchain_mut(&mut self);
			::std::convert::identity::<()>($body);
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
			::std::convert::identity::<()>($body);
			$self
		}
	};

	{
		$(#[$meta:meta])*
		$fn_name:ident$([$($generics:tt)*])?($nc:ident $(, $($args:tt)*)?) $(where { $($where_clause:tt)* })? => $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $(, $($args)*)?) -> Self $(where $($where_clause)*)? {
			let $nc = $crate::chainer::traits::ChainHalf::as_nonchain_mut(&mut self);
			::std::convert::identity::<()>($body);
			self
		}
	};
}
use chain_fn;
