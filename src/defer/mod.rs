use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{ Deref, DerefMut };
use std::ptr;
use std::thread::panicking;

#[macro_export]
macro_rules! defer {
	{ $($defer:tt)* } => {
		let __defer = $crate::defer::run_on_drop(move || { $($defer)* });
	}
}
pub use defer;

#[macro_export]
macro_rules! defer_success {
	{ $($defer:tt)* } => {
		let __defer = $crate::defer::run_on_success_drop(move || { $($defer)* });
	}
}
pub use defer_success;

#[macro_export]
macro_rules! defer_unwind {
	{ $($defer:tt)* } => {
		let __defer = $crate::defer::run_on_unwinding_drop(move || { $($defer)* });
	}
}
pub use defer_unwind;

#[must_use = "the deferred code would run immediately because rust drops the value immediately (if you only want to defer running some code, consider `defer!`, `defer_success!`, or `defer_unwind!`)"]
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

	pub struct Always;
	pub struct Success;
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
fn _mk_defer<W, F>(f: F) -> Defer<(), W, impl FnOnce(())>
where
	W: when::When,
	F: FnOnce()
{
	_mk_defer_with((), move |_| f())
}

fn _mk_defer_with<T, W, F>(item: T, f: F) -> Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	let item = ManuallyDrop::new(item);
	let f = ManuallyDrop::new(f);
	Defer { item, f, _when: PhantomData }
}

#[inline]
pub fn run_on_drop<F>(f: F) -> DeferAlways<(), impl FnOnce(())>
where
	F: FnOnce()
{
	_mk_defer(f)
}

#[inline]
pub fn run_on_drop_with<T, F>(item: T, f: F) -> DeferAlways<T, F>
where
	F: FnOnce(T)
{
	_mk_defer_with(item, f)
}

#[inline]
pub fn run_on_success_drop<F>(f: F) -> DeferSuccess<(), impl FnOnce(())>
where
	F: FnOnce()
{
	_mk_defer(f)
}

#[inline]
pub fn run_on_success_drop_with<T, F>(item: T, f: F) -> DeferSuccess<T, F>
where
	F: FnOnce(T)
{
	_mk_defer_with(item, f)
}

#[inline]
pub fn run_on_unwinding_drop<F>(f: F) -> DeferUnwind<(), impl FnOnce(())>
where
	F: FnOnce()
{
	_mk_defer(f)
}

#[inline]
pub fn run_on_unwinding_drop_with<T, F>(item: T, f: F) -> DeferUnwind<T, F>
where
	F: FnOnce(T)
{
	_mk_defer_with(item, f)
}

impl<T, W, F> Deref for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	type Target = T;
	fn deref(&self) -> &T {
		&self.item
	}
}

impl<T, W, F> DerefMut for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
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
		run_on_drop_with(self, f)
	}

	#[inline]
	fn on_success_drop<F>(self, f: F) -> DeferSuccess<Self, F>
	where
		F: FnOnce(Self)
	{
		run_on_success_drop_with(self, f)
	}

	#[inline]
	fn on_unwinding_drop<F>(self, f: F) -> DeferUnwind<Self, F>
	where
		F: FnOnce(Self)
	{
		run_on_unwinding_drop_with(self, f)
	}

	#[inline]
	fn defer<F>(self, f: F) -> DeferAlways<Self, F>
	where
		F: FnOnce(Self)
	{
		run_on_drop_with(self, f)
	}

	#[inline]
	fn defer_success<F>(self, f: F) -> DeferSuccess<Self, F>
	where
		F: FnOnce(Self)
	{
		run_on_success_drop_with(self, f)
	}

	#[inline]
	fn defer_unwind<F>(self, f: F) -> DeferUnwind<Self, F>
	where
		F: FnOnce(Self)
	{
		run_on_unwinding_drop_with(self, f)
	}
}

impl<T> OnDrop for T {}
