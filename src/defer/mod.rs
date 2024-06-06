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
	item: ManuallyDrop<T>,
	f: ManuallyDrop<F>,
	_when: PhantomData<W>
}

mod when {
	use super::*;

	pub trait When {
		fn run() -> bool;
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
	}

	impl When for Success {
		#[inline]
		fn run() -> bool { !panicking() }
	}

	impl When for Unwind {
		#[inline]
		fn run() -> bool { panicking() }
	}
}

pub type DeferAlways<T, F> = Defer<T, when::Always, F>;
pub type DeferSuccess<T, F> = Defer<T, when::Success, F>;
pub type DeferUnwind<T, F> = Defer<T, when::Unwind, F>;

#[inline]
fn _new_with<T, W, F>(item: T, f: F) -> Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	let item = ManuallyDrop::new(item);
	let f = ManuallyDrop::new(f);
	Defer { item, f, _when: PhantomData }
}

#[inline]
pub fn defer_with<T, F>(item: T, f: F) -> DeferAlways<T, F>
where
	F: FnOnce(T)
{
	_new_with(item, f)
}

#[inline]
pub fn defer_on_success_with<T, F>(item: T, f: F) -> DeferSuccess<T, F>
where
	F: FnOnce(T)
{
	_new_with(item, f)
}

#[inline]
pub fn defer_on_unwind_with<T, F>(item: T, f: F) -> DeferUnwind<T, F>
where
	F: FnOnce(T)
{
	_new_with(item, f)
}

impl<T, W, F> Deref for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	type Target = T;

	#[inline]
	fn deref(&self) -> &T {
		&self.item
	}
}

impl<T, W, F> DerefMut for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		&mut self.item
	}
}

impl<T, W, F> Drop for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	fn drop(&mut self) {
		unsafe {
			let item = ptr::read(&*self.item);
			let f = ptr::read(&*self.f);

			if !W::run() { return }
			f(item);
		}
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
