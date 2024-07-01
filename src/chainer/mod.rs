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

// construction, chaining, conversion, traits

// TODO: check callbacks and use chaining apis there? maybe?

macro_rules! chainer {
	(@doclink link: $doclink:literal, nonchain: $($nonchain:tt)+) => { $doclink };

	(@doclink nonchain: $($nonchain:tt)+) => { stringify!($($nonchain)+) };

	// implements PartialEq and PartialOrd
	{
		@gen_partial_cmp
		[$([$($generics_decl:tt)*])?]
		[$($nonchain:tt)+]

		[$($left:tt)+]
		[$left_expr:expr]

		[$($right:tt)+]
		[$right_expr:expr]
	} => {
		impl$(<$($generics_decl)*>)? ::std::cmp::PartialEq<$($right)+> for $($left)+
		where
			$($nonchain)+: ::std::cmp::PartialEq<$($nonchain)+>
		{
			#[inline]
			fn eq(&self, other: &$($right)+) -> ::std::primitive::bool {
				<$($nonchain)+ as ::std::cmp::PartialEq<$($nonchain)+>>::eq($left_expr(self), $right_expr(other))
			}

			// overriding this in case nonchain has overridden it
			#[allow(clippy::partialeq_ne_impl)]
			#[inline]
			fn ne(&self, other: &$($right)+) -> ::std::primitive::bool {
				<$($nonchain)+ as ::std::cmp::PartialEq<$($nonchain)+>>::ne($left_expr(self), $right_expr(other))
			}
		}

		impl$(<$($generics_decl)*>)? ::std::cmp::PartialOrd<$($right)+> for $($left)+
		where
			$($nonchain)+: ::std::cmp::PartialOrd<$($nonchain)+>
		{
			#[inline]
			fn partial_cmp(&self, other: &$($right)+) -> ::std::option::Option<::std::cmp::Ordering> {
				<$($nonchain)+ as ::std::cmp::PartialOrd<$($nonchain)+>>::partial_cmp($left_expr(self), $right_expr(other))
			}

			#[inline]
			fn lt(&self, other: &$($right)+) -> ::std::primitive::bool {
				<$($nonchain)+ as ::std::cmp::PartialOrd<$($nonchain)+>>::lt($left_expr(self), $right_expr(other))
			}

			#[inline]
			fn le(&self, other: &$($right)+) -> ::std::primitive::bool {
				<$($nonchain)+ as ::std::cmp::PartialOrd<$($nonchain)+>>::le($left_expr(self), $right_expr(other))
			}

			#[inline]
			fn gt(&self, other: &$($right)+) -> ::std::primitive::bool {
				<$($nonchain)+ as ::std::cmp::PartialOrd<$($nonchain)+>>::gt($left_expr(self), $right_expr(other))
			}

			#[inline]
			fn ge(&self, other: &$($right)+) -> ::std::primitive::bool {
				<$($nonchain)+ as ::std::cmp::PartialOrd<$($nonchain)+>>::ge($left_expr(self), $right_expr(other))
			}
		}
	};

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

		// we follow whatever inner is doing (which _would_ include incorrect behaviour...)
		#[allow(clippy::non_canonical_clone_impl)]
		impl$(<$($generics_decl)*>)? ::std::clone::Clone for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::clone::Clone
		{
			#[inline]
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
					.field("_", $crate::chainer::traits::ChainHalf::as_nonchain(self))
					.finish()
			}
		}

		impl$(<$($generics_decl)*>)? ::std::default::Default for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::default::Default
		{
			#[inline]
			fn default() -> Self {
				<$($nonchain)+ as ::std::default::Default>::default().into()
			}
		}

		impl$(<$($generics_decl)*>)? ::std::fmt::Display for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::fmt::Display
		{
			#[inline]
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				<$($nonchain)+ as ::std::fmt::Display>::fmt(self.as_nonchain(), f)
			}
		}

		impl$(<$($generics_decl)*>)? ::std::cmp::Eq for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::cmp::Eq
		{}

		impl$(<$($generics_decl)*>)? ::std::cmp::Ord for $chainer$(<$($generics)*>)?
		where
			$($nonchain)+: ::std::cmp::Ord
		{
			#[inline]
			fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
				<$($nonchain)+ as ::std::cmp::Ord>::cmp(self.as_nonchain(), other.as_nonchain())
			}

			#[inline]
			fn max(self, other: Self) -> Self {
				<$($nonchain)+ as ::std::cmp::Ord>::max(self.into_nonchain(), other.into_nonchain()).into()
			}

			#[inline]
			fn min(self, other: Self) -> Self {
				<$($nonchain)+ as ::std::cmp::Ord>::min(self.into_nonchain(), other.into_nonchain()).into()
			}

			#[inline]
			fn clamp(self, min: Self, max: Self) -> Self {
				<$($nonchain)+ as ::std::cmp::Ord>::clamp(self.into_nonchain(), min.into_nonchain(), max.into_nonchain()).into()
			}
		}

		// chain / chain
		$crate::chainer::chainer! {
			@gen_partial_cmp
			[$([$($generics_decl)*])?]
			[$($nonchain)+]

			[$chainer$(<$($generics)*>)?]
			[$crate::chainer::traits::ChainHalf::as_nonchain]

			[$chainer$(<$($generics)*>)?]
			[$crate::chainer::traits::ChainHalf::as_nonchain]
		}

		// chain / nonchain
		$crate::chainer::chainer! {
			@gen_partial_cmp
			[$([$($generics_decl)*])?]
			[$($nonchain)+]

			[$($nonchain)+]
			[::std::convert::identity]

			[$chainer$(<$($generics)*>)?]
			[$crate::chainer::traits::ChainHalf::as_nonchain]
		}

		// nonchain / chain
		$crate::chainer::chainer! {
			@gen_partial_cmp
			[$([$($generics_decl)*])?]
			[$($nonchain)+]

			[$chainer$(<$($generics)*>)?]
			[$crate::chainer::traits::ChainHalf::as_nonchain]

			[$($nonchain)+]
			[::std::convert::identity]
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
