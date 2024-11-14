//! Function traits like Rust's [`Fn`], [`FnMut`], and [`FnOnce`] traits
//!
//! "why" you may be wondering? because, these are traits I can implement on my
//! own types without using unstable feature flags, then I can use these traits
//! in trait bounds, and they will accept items implementing rust's core function
//! traits (eg. functions and closures), and my own custom, nameable types.

use crate::prelude_std::*;
use crate::macro_util::macro_recurse;

/// Function trait akin to rust's [`Fn`]
///
/// This trait is implemented for all [`Fn`] types with up to 16 arguments.
pub trait Function<Args = (), Return = ()> {
	fn call(&self, args: Args) -> Return;
}

/// Function trait akin to rust's [`FnMut`]
///
/// This trait is implemented for all [`FnMut`] types with up to 16 arguments.
pub trait FunctionMut<Args = (), Return = ()> {
	fn call_mut(&mut self, args: Args) -> Return;
}

/// Function trait akin to rust's [`FnOnce`]
///
/// This trait is implemented for all [`FnOnce`] types with up to 16 arguments.
pub trait FunctionOnce<Args = (), Return = ()> {
	fn call_once(self, args: Args) -> Return;
}

macro_rules! impl_fn {
	{
		$trait_name:ident
		$std_trait:ident
		$fn_name:ident($self_ident:ident $($self:tt)*)
		$first:ident $($type_params:ident)*
	} => {
		macro_recurse! {
			@exclude_zero
			impl_fn
			{
				$trait_name
				$std_trait
				$fn_name($self_ident $($self)*)
			}
			{ $first $($type_params)* }
		}
	};

	{
		@wiwi_macro_recurse
		{
			$trait_name:ident
			$std_trait:ident
			$fn_name:ident($self_ident:ident $($self:tt)*)
		}
		{ $($type_params:ident)* }
	} => {
		#[doc(hidden)]
		impl<F, R, $($type_params),*> $trait_name<($($type_params,)*), R> for F
		where
			F: $std_trait($($type_params),*) -> R
		{
			#[inline]
			#[allow(non_snake_case, reason = "macro output")]
			#[expect(
				clippy::allow_attributes,
				reason = "macro output may or may not trigger non_snake_case, so can't use `expect`"
			)]
			fn $fn_name($($self)*, ($($type_params,)*): ($($type_params,)*)) -> R {
				$self_ident($($type_params),*)
			}
		}
	};
}

impl<F, R> Function<(), R> for F
where
	F: Fn() -> R
{
	#[inline]
	fn call(&self, _: ()) -> R {
		self()
	}
}

impl<F, R> FunctionMut<(), R> for F
where
	F: FnMut() -> R
{
	#[inline]
	fn call_mut(&mut self, _: ()) -> R {
		self()
	}
}

impl<F, R> FunctionOnce<(), R> for F
where
	F: FnOnce() -> R
{
	#[inline]
	fn call_once(self, _: ()) -> R {
		self()
	}
}

impl_fn! {
	Function
	Fn
	call(self &self)
	A1 A2 A3 A4
	A5 A6 A7 A8
	A9 A10 A11 A12
	A13 A14 A15 A16
}

impl_fn! {
	FunctionMut
	FnMut
	call_mut(self &mut self)
	A1 A2 A3 A4
	A5 A6 A7 A8
	A9 A10 A11 A12
	A13 A14 A15 A16
}

impl_fn! {
	FunctionOnce
	FnOnce
	call_once(self self)
	A1 A2 A3 A4
	A5 A6 A7 A8
	A9 A10 A11 A12
	A13 A14 A15 A16
}

#[repr(transparent)]
pub struct FnAdapter<F, Args = (), Return = ()> {
	f: F,
	__marker: PhantomData<fn(Args) -> Return>
}

impl<F, Args, Return> FnAdapter<F, Args, Return>
where
	F: Function<Args, Return>
{
	#[inline]
	pub fn new(f: F) -> Self {
		Self { f, __marker: PhantomData }
	}

	#[inline]
	pub fn into_inner(self) -> F {
		self.f
	}
}

impl<F, Args, Return> Function<Args, Return> for FnAdapter<F, Args, Return>
where
	F: Function<Args, Return>
{
	#[inline]
	fn call(&self, args: Args) -> Return {
		self.f.call(args)
	}
}

impl<F, Args, Return> FunctionMut<Args, Return> for FnAdapter<F, Args, Return>
where
	F: Function<Args, Return>
{
	#[inline]
	fn call_mut(&mut self, args: Args) -> Return {
		self.f.call(args)
	}
}

impl<F, Args, Return> FunctionOnce<Args, Return> for FnAdapter<F, Args, Return>
where
	F: Function<Args, Return>
{
	#[inline]
	fn call_once(self, args: Args) -> Return {
		self.f.call(args)
	}
}

#[repr(transparent)]
pub struct FnMutAdapter<F, Args = (), Return = ()> {
	f: F,
	__marker: PhantomData<fn(Args) -> Return>
}

impl<F, Args, Return> FnMutAdapter<F, Args, Return>
where
	F: FunctionMut<Args, Return>
{
	#[inline]
	pub fn new(f: F) -> Self {
		Self { f, __marker: PhantomData }
	}

	#[inline]
	pub fn into_inner(self) -> F {
		self.f
	}
}

impl<F, Args, Return> FunctionMut<Args, Return> for FnMutAdapter<F, Args, Return>
where
	F: FunctionMut<Args, Return>
{
	#[inline]
	fn call_mut(&mut self, args: Args) -> Return {
		self.f.call_mut(args)
	}
}

impl<F, Args, Return> FunctionOnce<Args, Return> for FnMutAdapter<F, Args, Return>
where
	F: FunctionMut<Args, Return>
{
	#[inline]
	fn call_once(mut self, args: Args) -> Return {
		self.f.call_mut(args)
	}
}
