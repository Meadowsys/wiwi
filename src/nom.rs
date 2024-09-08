//! Utilities for nominal typing

use crate::ptr::coerce_ptr;
use crate::rust_std::clone::Clone;
use crate::rust_std::cmp::{ Eq, Ord, PartialEq, PartialOrd, Ordering };
use crate::rust_std::convert::{ From, Into };
use crate::rust_std::default::Default;
use crate::rust_std::fmt::{ self, Formatter };
use crate::rust_std::fmt::{ Debug, Display };
use crate::rust_std::future::IntoFuture;
use crate::rust_std::hash::{ Hash, Hasher };
use crate::rust_std::marker::{ Copy, PhantomData, Sized };
use crate::rust_std::ops::FnOnce;
use crate::rust_std::option::Option;
use crate::rust_std::result::Result;
use crate::rust_std::slice;

/// Declare a new nominal type (alias), with the provided name, a name for the
/// marker type struct, and the wrapped type
///
/// The returned type alias will be of a guaranteed unique type. This is done by
/// creating a new ZST with the provided marker type struct name.
///
/// The name of the new marker type struct is something we hope to eventually be
/// able to generate automatically from the given newtype name. If there is a way,
/// we don't know how >~<
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use wiwi::nom::{ Nom, nom };
/// // type NewType = Nom<String, NewTypeMarker>;
/// nom!(NewType, marker: NewTypeMarker, wraps: String);
///
/// // these two are identical
/// let item: NewType = NewType::new(String::new());
/// let item: Nom<String, NewTypeMarker> = Nom::new(String::new());
///
/// // and of course, it's a type alias
/// let item: NewType = Nom::new(String::new());
/// let item: Nom<String, NewTypeMarker> = NewType::new(String::new());
/// ```
///
/// The macro does indeed create a unique newtype:
///
/// ```compile_fail
/// # // TODO: use err code E0308 (currently nightly only)
/// # use wiwi::nom::nom;
/// # nom!(NewType, marker: NewTypeMarker, wraps: String);
/// nom!(AnotherNewType, marker: AnotherNewTypeMarker, wraps: String);
///
/// let item: NewType = NewType::new(String::new());
/// // this won't compile
/// let another_item: NewType = AnotherNewType::new(String::new());
/// ```
///
/// Controlling visibility of the type alias / marker struct:
///
/// ```
/// mod inner {
///    # use wiwi::nom::nom;
///    nom!(pub NewType, marker: NewTypeMarker, wraps: String);
///    //   ↑
/// }
///
/// let item = inner::NewType::new(String::new());
/// ```
///
/// Private visibility is default (like the rest of Rust visibilities):
///
/// ```compile_fail
/// # // TODO: use err code E0603 (currently nightly only)
/// mod inner {
///    # use wiwi::nom::nom;
///    nom!(NewType, marker: NewTypeMarker, wraps: String);
///    //       ↑ no `pub`
/// }
///
/// // this won't compile
/// let item = inner::NewType::new(String::new());
/// ```
///
/// Other visibilities work too, of course:
///
/// ```compile_fail
/// # // TODO: use err code E0603 (currently nightly only)
/// mod outer {
///    mod inner {
///       # // pag
///       # use wiwi::nom::nom;
///       nom!(pub(super) NewType, marker: NewTypeMarker, wraps: String);
///    }
///
///    # fn _maybe_this_fn_decl_shouldnt_be_hidden_i_dont_know() {
///    // this is fine...
///    let item = inner::NewType::new(String::new());
///    # }
/// }
///
/// // but this won't compile
/// let item = outer::inner::NewType::new(String::new());
/// ```
#[macro_export]
macro_rules! nom {
	($vis:vis $name:ident, marker: $marker:ident, wraps: $( ref <$($lifetimes:lifetime),+> )? $ty:ty) => {
		$vis struct $marker;
		$vis type $name$(<$($lifetimes),+>)? = $crate::nom::Nom<$ty, $marker>;
	};
}
pub use nom;

/// Declare many new nominal types (aliases), in a module
///
/// Usage is more or less identical to [`nom`], but you define a module
/// inside the macro invocation. Because this macro creates a new module
/// (with the name you specify), and the created module is only used for defining
/// these
/// nominal types, there can be nothing else in there, which we take advantage of
/// to create a `marker` submodule to define marker types in. This way it can have
/// a new namespace just for the marker types, so reusing the newtype name won't
/// collide with anything else.
///
/// So, all of that is to say this macro also saves you a bit of boilerplate
/// declaring names for the newtype ZSTs.
///
/// # Examples
///
/// ```
/// # use wiwi::nom::nom_mod;
/// nom_mod! {
///    pub mod nom {
///       nom!(pub NewType, wraps: String);
///    }
/// }
///
/// let item = nom::NewType::new(String::new());
/// ```
///
/// Still creating newtypes as expected:
///
/// ```compile_fail
/// # // TODO: use err code E0308 (currently nightly only)
/// # use wiwi::nom::nom_mod;
/// nom_mod! {
///    pub mod nom {
///       nom!(pub NewType, wraps: String);
///       nom!(pub AnotherNewType, wraps: String);
///    }
/// }
///
/// let item: nom::NewType = nom::NewType::new(String::new());
/// // this won't compile
/// let another_item: nom::NewType = nom::AnotherNewType::new(String::new());
/// ```
///
/// Still "just" a type alias:
///
/// ```
/// # use wiwi::nom::{ Nom, nom_mod };
/// # nom_mod! {
/// #    pub mod nom {
/// #       nom!(pub NewType, wraps: String);
/// #    }
/// # }
///
/// let item: nom::NewType = Nom::new(String::new());
/// ```
///
/// Created marker structs are in a `marker` submodule:
///
/// ```
/// # use wiwi::nom::{ Nom, nom_mod };
/// # nom_mod! {
/// #    pub mod nom {
/// #       nom!(pub NewType, wraps: String);
/// #       nom!(pub AnotherNewType, wraps: String);
/// #    }
/// # }
///
/// let item: Nom<String, nom::marker::NewType> = nom::NewType::new(String::new());
/// let item: Nom<String, nom::marker::AnotherNewType> = nom::AnotherNewType::new(String::new());
/// ```
#[macro_export]
macro_rules! nom_mod {
	{
		$(
			$mod_vis:vis mod $mod_name:ident {
				$( nom!($item_vis:vis $name:ident, wraps: $( ref <$($lifetimes:lifetime),+> )? $type:ty); )*
			}
		)*
	} => {
		$(
			$mod_vis mod $mod_name {
				pub mod marker {
					$( pub struct $name; )*
				}

				use super::*;
				$( $item_vis type $name$( <$($lifetimes),+> )? = $crate::nom::Nom<$type, marker::$name>; )*
			}
		)*
	}
}
pub use nom_mod;

/// Nominal wrapper struct
///
/// This struct consists of a value `T` and a marker [`PhantomData<M>`]. It does
/// not incur any overhead for the marker type; it is simply a type safe wrapper.
///
/// Newtypes are primarily created with assistance from the [`nom`] and
/// [`nom_mod`] macros. The macros will help save you the boilerplate of
/// writing the types and declaring unit structs to use as the marker.
#[repr(transparent)]
pub struct Nom<T, M> {
	/// The wrapped item
	item: T,

	/// Marker zero-sized field for the marker type parameter `M`
	marker: PhantomData<M>
}

impl<T, M> Nom<T, M> {
	/// Creates a nominal struct with the given value
	#[inline]
	pub fn new(item: T) -> Self {
		Self { item, marker: PhantomData }
	}

	/// Unwraps the nominal struct and returns the value
	///
	/// Note: I didn't call this `unwrap` because it very much appears to be the
	/// same method as on [`Option`] and [`Result`], which implies panicking, but
	/// this method will not panick.
	// TODO: was into_inner better?
	#[inline]
	pub fn unwrap_value(self) -> T {
		self.item
	}

	/// Gets a reference to the wrapped value
	///
	/// Note: [`Deref`](std::ops::Deref) is not implemented on purpose,
	/// to prevent unintentional auto-derefs
	// TODO: should we reconsider the above?
	#[inline]
	pub fn wrapped_ref(&self) -> &T {
		&self.item
	}

	/// Gets a mut reference to the wrapped value
	///
	/// Note: [`DerefMut`](std::ops::DerefMut) is not implemented on purpose,
	/// to prevent unintentional auto-derefs
	// TODO: should we reconsider the above?
	#[inline]
	pub fn wrapped_mut(&mut self) -> &mut T {
		&mut self.item
	}

	/// Unwraps and rewraps the value as another nominal type, without modifying
	/// the wrapped value
	///
	/// If you're using this function, make sure you know why you're using it!
	/// after all, the whole point of this is to seperate otherwise identical
	/// types into newtypes based on semantic meaning.
	#[inline]
	pub fn transmute_wrapper<M2>(self) -> Nom<T, M2> {
		Nom::new(self.unwrap_value())
	}

	/// Consumes and "map"s the wrapped value into another value, wrapping it in
	/// a nominal type with the same marker
	#[inline]
	pub fn map_value<T2, F>(self, f: F) -> Nom<T2, M>
	where
		F: FnOnce(T) -> T2
	{
		Nom::new(f(self.unwrap_value()))
	}

	/// Maps the wrapped value and transmutes the wrapper type
	///
	/// Effectively the same as calling [`map_value`] and then [`transmute_wrapper`].
	///
	/// [`map_value`]: Self::map_value
	/// [`transmute_wrapper`]: Self::transmute_wrapper
	#[inline]
	pub fn map_transmute<T2, M2, F>(self, f: F) -> Nom<T2, M2>
	where
		F: FnOnce(T) -> T2
	{
		self.map_value(f).transmute_wrapper()
	}

	/// Consumes and asyncronously "map"s the wrapped value into another value,
	/// wrapping it in a nominal type with the same marker
	#[inline]
	pub async fn async_map_value<T2, F, Fu>(self, f: F) -> Nom<T2, M>
	where
		F: FnOnce(T) -> Fu,
		Fu: IntoFuture<Output = T2>
	{
		Nom::new(f(self.unwrap_value()).await)
	}

	/// Asyncronously maps the wrapped value and transmutes the wrapper type
	///
	/// Effectively the same as calling [`async_map_value`] and then [`transmute_wrapper`].
	///
	/// [`async_map_value`]: Self::async_map_value
	/// [`transmute_wrapper`]: Self::transmute_wrapper
	#[inline]
	pub async fn async_map_transmute<T2, M2, F, Fu>(self, f: F) -> Nom<T2, M>
	where
		F: FnOnce(T) -> Fu,
		Fu: IntoFuture<Output = T2>
	{
		self.async_map_value(f).await.transmute_wrapper()
	}
}

impl<T, M, E> Nom<Result<T, E>, M> {
	/// Transpose a nominal wrapped [`Result`] into a [`Result`] of a nominal
	/// wrapped value
	///
	/// The value gets wrapped, but the error does not. Both are not otherwise
	/// modified in any way.
	#[inline]
	pub fn transpose(self) -> Result<Nom<T, M>, E> {
		self.unwrap_value().map(Nom::new)
	}

	/// Maps the [`Ok`] value of a [`Result`], wrapping the resulting [`Result`]
	/// in a nominal type with the same marker
	#[inline]
	pub fn map_result_ok<T2, F>(self, f: F) -> Nom<Result<T2, E>, M>
	where
		F: FnOnce(T) -> T2
	{
		Nom::new(self.unwrap_value().map(f))
	}

	/// Maps the [`Err`] value of a [`Result`], wrapping the resulting [`Result`]
	/// in a nominal type with the same marker
	#[inline]
	pub fn map_result_err<E2, F>(self, f: F) -> Nom<Result<T, E2>, M>
	where
		F: FnOnce(E) -> E2
	{
		Nom::new(self.unwrap_value().map_err(f))
	}
}

impl<T, M> Nom<Option<T>, M> {
	/// Transpose a nominal wrapped [`Option`] into an [`Option`] of a nominal
	/// wrapped value
	///
	/// The value is not otherwise modified in any way.
	#[inline]
	pub fn transpose(self) -> Option<Nom<T, M>> {
		self.unwrap_value().map(Nom::new)
	}

	/// Maps the [`Some`] value of an [`Option`], wrapping the resulting [`Option`]
	/// in a nominal type with the same marker
	#[inline]
	pub fn map_option_some<T2, F>(self, f: F) -> Nom<Option<T2>, M>
	where
		F: FnOnce(T) -> T2
	{
		Nom::new(self.unwrap_value().map(f))
	}
}

impl<T, M> From<T> for Nom<T, M> {
	#[inline]
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

// delegate trait impls by just calling T's impl

impl<T: Clone, M> Clone for Nom<T, M> {
	#[inline]
	fn clone(&self) -> Self {
		self.wrapped_ref().clone().into()
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		self.wrapped_mut().clone_from(source.wrapped_ref())
	}
}

impl<T: Copy, M> Copy for Nom<T, M> {}

impl<T: Debug, M> Debug for Nom<T, M> {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Nom")
			.field("value", self.wrapped_ref())
			.finish()
	}
}

impl<T: Display, M> Display for Nom<T, M> {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.wrapped_ref().fmt(f)
	}
}

impl<T: Default, M> Default for Nom<T, M> {
	#[inline]
	fn default() -> Self {
		T::default().into()
	}
}

impl<T: Hash, M> Hash for Nom<T, M> {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.wrapped_ref().hash(state)
	}

	#[inline]
	fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
	where
		Self: Sized
	{
		let t_data = coerce_ptr(data).cast::<T>();

		// SAFETY: we're repr(transparent)
		let t_data = unsafe { slice::from_raw_parts(t_data, data.len()) };

		T::hash_slice(t_data, state)
	}
}

impl<T: PartialEq<TR>, M, TR, MR> PartialEq<Nom<TR, MR>> for Nom<T, M> {
	#[inline]
	fn eq(&self, other: &Nom<TR, MR>) -> bool {
		self.wrapped_ref().eq(other.wrapped_ref())
	}

	// we override ne here since T might have overridden ne,
	// and we should use it if so
	#[allow(clippy::partialeq_ne_impl)]
	#[inline]
	fn ne(&self, other: &Nom<TR, MR>) -> bool {
		self.wrapped_ref().ne(other.wrapped_ref())
	}
}

impl<T: Eq, M> Eq for Nom<T, M> {}

impl<T: PartialOrd<TR>, M, TR, MR> PartialOrd<Nom<TR, MR>> for Nom<T, M> {
	#[inline]
	fn partial_cmp(&self, other: &Nom<TR, MR>) -> Option<Ordering> {
		self.wrapped_ref().partial_cmp(other.wrapped_ref())
	}

	#[inline]
	fn lt(&self, other: &Nom<TR, MR>) -> bool {
		self.wrapped_ref().lt(other.wrapped_ref())
	}

	#[inline]
	fn le(&self, other: &Nom<TR, MR>) -> bool {
		self.wrapped_ref().le(other.wrapped_ref())
	}

	#[inline]
	fn gt(&self, other: &Nom<TR, MR>) -> bool {
		self.wrapped_ref().gt(other.wrapped_ref())
	}

	#[inline]
	fn ge(&self, other: &Nom<TR, MR>) -> bool {
		self.wrapped_ref().ge(other.wrapped_ref())
	}
}

impl<T: Ord, M> Ord for Nom<T, M> {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		self.wrapped_ref().cmp(other.wrapped_ref())
	}

	#[inline]
	fn max(self, other: Self) -> Self {
		self.unwrap_value().max(other.unwrap_value()).into()
	}

	#[inline]
	fn min(self, other: Self) -> Self {
		self.unwrap_value().min(other.unwrap_value()).into()
	}

	#[inline]
	fn clamp(self, min: Self, max: Self) -> Self {
		self.unwrap_value().clamp(min.unwrap_value(), max.unwrap_value()).into()
	}
}
