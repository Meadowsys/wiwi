use std::fmt::{ self, Debug, Display };
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{ Deref, DerefMut };
use std::ptr;
use std::thread::panicking;

#[macro_export]
macro_rules! defer {
	{ $($defer:tt)* } => {
		let __defer = <() as $crate::defer::OnDrop>::on_drop((), move |()| { $($defer)* });
	}
}
pub use defer;

#[macro_export]
macro_rules! defer_success {
	{ $($defer:tt)* } => {

		let __defer = <() as $crate::defer::OnDrop>::on_success_drop((), move |()| { $($defer)* });
	}
}
pub use defer_success;

#[macro_export]
macro_rules! defer_unwind {
	{ $($defer:tt)* } => {

		let __defer = <() as $crate::defer::OnDrop>::on_unwind_drop((), move |()| { $($defer)* });
	}
}
pub use defer_unwind;

#[must_use = "the code intended to be deferred would run immediately because rust drops the value immediately (if you only want to defer running some code, consider `defer!` or its success/unwind variants)"]
pub struct Defer<T, W: when::When, F = fn(T)>
where
	F: FnOnce(T)
{
	value: ManuallyDrop<T>,
	f: ManuallyDrop<F>,
	_when: PhantomData<W>
}

mod when {
	use super::*;

	pub trait When: Debug {
		fn run() -> bool;
		fn construct_for_debug() -> Self;
	}

	#[derive(Debug)]
	pub struct Always;
	#[derive(Debug)]
	pub struct Success;
	#[derive(Debug)]
	pub struct Unwind;

	impl When for Always {
		#[inline]
		fn run() -> bool { true }
		#[inline]
		fn construct_for_debug() -> Self { Self }
	}

	impl When for Success {
		#[inline]
		fn run() -> bool { !panicking() }
		#[inline]
		fn construct_for_debug() -> Self { Self }
	}

	impl When for Unwind {
		#[inline]
		fn run() -> bool { panicking() }
		#[inline]
		fn construct_for_debug() -> Self { Self }
	}
}

pub type DeferAlways<T, F> = Defer<T, when::Always, F>;
pub type DeferSuccess<T, F> = Defer<T, when::Success, F>;
pub type DeferUnwind<T, F> = Defer<T, when::Unwind, F>;

#[inline]
fn _new_with<T, W, F>(value: T, f: F) -> Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	let value = ManuallyDrop::new(value);
	let f = ManuallyDrop::new(f);
	Defer { value, f, _when: PhantomData }
}

#[inline]
pub fn defer_with<T, F>(value: T, f: F) -> DeferAlways<T, F>
where
	F: FnOnce(T)
{
	_new_with(value, f)
}

#[inline]
pub fn defer_on_success_with<T, F>(value: T, f: F) -> DeferSuccess<T, F>
where
	F: FnOnce(T)
{
	_new_with(value, f)
}

#[inline]
pub fn defer_on_unwind_with<T, F>(value: T, f: F) -> DeferUnwind<T, F>
where
	F: FnOnce(T)
{
	_new_with(value, f)
}

impl<T, W, F> Deref for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	type Target = T;

	#[inline]
	fn deref(&self) -> &T {
		&self.value
	}
}

impl<T, W, F> DerefMut for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		&mut self.value
	}
}

impl<T, W, F> Drop for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	fn drop(&mut self) {
		unsafe {
			let value = ptr::read(&*self.value);
			let f = ptr::read(&*self.f);

			if !W::run() { return }
			f(value);
		}
	}
}

impl<T, W, F> Debug for Defer<T, W, F>
where
	T: Debug,
	W: when::When,
	F: FnOnce(T)
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Defer")
			.field("value", &self.value)
			.field("when", &W::construct_for_debug())
			.finish_non_exhaustive()
	}
}

impl<T, W, F> Display for Defer<T, W, F>
where
	T: Display,
	W: when::When,
	F: FnOnce(T)
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.value.fmt(f)
	}
}

impl<T, W, F, U> AsRef<U> for Defer<T, W, F>
where
	T: AsRef<U>,
	W: when::When,
	F: FnOnce(T)
{
	fn as_ref(&self) -> &U {
		self.value.as_ref()
	}
}

impl<T, W, F, U> AsMut<U> for Defer<T, W, F>
where
	T: AsMut<U>,
	W: when::When,
	F: FnOnce(T)
{
	fn as_mut(&mut self) -> &mut U {
		self.value.as_mut()
	}
}


pub trait OnDrop: Sized {
	#[inline]
	fn on_drop<F>(self, f: F) -> DeferAlways<Self, F>
	where
		F: FnOnce(Self)
	{
		_new_with(self, f)
	}

	#[inline]
	fn on_success_drop<F>(self, f: F) -> DeferSuccess<Self, F>
	where
		F: FnOnce(Self)
	{
		_new_with(self, f)
	}

	#[inline]
	fn on_unwind_drop<F>(self, f: F) -> DeferUnwind<Self, F>
	where
		F: FnOnce(Self)
	{
		_new_with(self, f)
	}
}

impl<T> OnDrop for T {}
