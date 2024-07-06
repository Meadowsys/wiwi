use std::fmt::{ self, Debug, Display };
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{ Deref, DerefMut };
use std::thread::panicking;

#[macro_export]
macro_rules! defer {
	{ move $($defer:tt)* } => {
		let __defer = $crate::defer::DeferAlways::new((), move |()| { $($defer)* });
	};

	{ $($defer:tt)* } => {
		let __defer = $crate::defer::DeferAlways::new((), |()| { $($defer)* });
	};
}
pub use defer;

#[macro_export]
macro_rules! defer_success {
	{ move $($defer:tt)* } => {
		let __defer = $crate::defer::DeferSuccess::new((), move |()| { $($defer)* });
	};

	{ $($defer:tt)* } => {
		let __defer = $crate::defer::DeferSuccess::new((), |()| { $($defer)* });
	};
}
pub use defer_success;

#[macro_export]
macro_rules! defer_unwind {
	{ move $($defer:tt)* } => {
		let __defer = $crate::defer::DeferUnwind::new((), move |()| { $($defer)* });
	};

	{ $($defer:tt)* } => {
		let __defer = $crate::defer::DeferUnwind::new((), |()| { $($defer)* });
	};
}
pub use defer_unwind;

#[must_use = "the code intended to be deferred would run immediately because rust drops the value immediately (if you only want to defer running some code, consider `defer!` or its success/unwind variants)"]
pub struct Defer<T, W: when::When, F = fn(T)>
where
	F: FnOnce(T)
{
	value: ManuallyDrop<T>,
	when: ManuallyDrop<W>,
	f: ManuallyDrop<F>
}

mod when {
	use super::*;

	pub trait When {
		fn should_run(self) -> bool;
	}

	#[derive(Debug)]
	pub struct Always;

	impl When for Always {
		#[inline]
		fn should_run(self) -> bool { true }
	}

	#[derive(Debug)]
	pub struct Success;

	impl When for Success {
		#[inline]
		fn should_run(self) -> bool { !panicking() }
	}

	#[derive(Debug)]
	pub struct Unwind;

	impl When for Unwind {
		#[inline]
		fn should_run(self) -> bool { panicking() }
	}

	#[derive(Debug)]
	pub struct Runtime {
		pub(super) should_run: bool
	}

	impl When for Runtime {
		#[inline]
		fn should_run(self) -> bool { self.should_run }
	}

	pub struct RuntimeFn<T, F = fn(T) -> bool>
	where
		F: FnOnce(T) -> bool
	{
		pub(super) value: ManuallyDrop<T>,
		pub(super) f: ManuallyDrop<F>
	}

	impl<T, F> Debug for RuntimeFn<T, F>
	where
		F: FnOnce(T) -> bool,
		T: Debug
	{
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Defer<Runtime>")
				.field("value", &*self.value)
				.finish_non_exhaustive()
		}
	}

	impl<T, F> When for RuntimeFn<T, F>
	where
		F: FnOnce(T) -> bool
	{
		#[inline]
		fn should_run(self) -> bool {
			unsafe {
				let mut me = ManuallyDrop::new(self);
				let value = ManuallyDrop::take(&mut me.value);
				let f = ManuallyDrop::take(&mut me.f);
				f(value)
			}
		}
	}

	impl<T, F> Drop for RuntimeFn<T, F>
	where
		F: FnOnce(T) -> bool
	{
		fn drop(&mut self) {
			unsafe {
				ManuallyDrop::drop(&mut self.value);
				ManuallyDrop::drop(&mut self.f);
			}
		}
	}
}

pub type DeferAlways<T, F = fn(T)> = Defer<T, when::Always, F>;
pub type DeferSuccess<T, F = fn(T)> = Defer<T, when::Success, F>;
pub type DeferUnwind<T, F = fn(T)> = Defer<T, when::Unwind, F>;
pub type DeferRuntime<T, F = fn(T)> = Defer<T, when::Runtime, F>;
pub type DeferRuntimeFn<
	T,
	Twhen,
	F = fn(T),
	Fwhen = fn(Twhen) -> bool
> = Defer<T, when::RuntimeFn<Twhen, Fwhen>, F>;

impl<T, W, F> Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn _new(value: T, f: F, when: W) -> Self {
		let value = ManuallyDrop::new(value);
		let when = ManuallyDrop::new(when);
		let f = ManuallyDrop::new(f);
		Defer { value, when, f }
	}

	#[inline]
	fn _replace_when<W2>(self, when: W2) -> Defer<T, W2, F>
	where
		W2: when::When
	{
		unsafe {
			let mut me = ManuallyDrop::new(self);

			let value = ManuallyDrop::take(&mut me.value);
			let f = ManuallyDrop::take(&mut me.f);
			ManuallyDrop::drop(&mut me.when);

			Defer::_new(value, f, when)
		}
	}

	#[inline]
	pub fn into_always(self) -> DeferAlways<T, F> {
		self._replace_when(when::Always)
	}

	#[inline]
	pub fn into_on_success(self) -> DeferSuccess<T, F> {
		self._replace_when(when::Success)
	}

	#[inline]
	pub fn into_on_unwind(self) -> DeferUnwind<T, F> {
		self._replace_when(when::Unwind)
	}

	#[inline]
	pub fn into_runtime(self, should_run: bool) -> DeferRuntime<T, F> {
		self._replace_when(when::Runtime { should_run })
	}

	#[inline]
	pub fn into_runtime_fn<Twhen, Fwhen>(
		self,
		should_run_value: Twhen,
		should_run: Fwhen
	) -> DeferRuntimeFn<T, Twhen, F, Fwhen>
	where
		Fwhen: FnOnce(Twhen) -> bool
	{
		let value = ManuallyDrop::new(should_run_value);
		let f = ManuallyDrop::new(should_run);
		self._replace_when(when::RuntimeFn { value, f })
	}
}

impl<T, F> DeferAlways<T, F>
where
	F: FnOnce(T)
{
	#[inline]
	pub fn new(value: T, f: F) -> Self {
		Self::_new(value, f, when::Always)
	}
}

impl<T, F> DeferSuccess<T, F>
where
	F: FnOnce(T)
{
	#[inline]
	pub fn new(value: T, f: F) -> Self {
		Self::_new(value, f, when::Success)
	}
}

impl<T, F> DeferUnwind<T, F>
where
	F: FnOnce(T)
{
	#[inline]
	pub fn new(value: T, f: F) -> Self {
		Self::_new(value, f, when::Unwind)
	}
}

impl<T, F> DeferRuntime<T, F>
where
	F: FnOnce(T)
{
	#[inline]
	pub fn new(value: T, f: F, should_run: bool) -> Self {
		Self::_new(value, f, when::Runtime { should_run })
	}
}

impl<T, Twhen, F, Fwhen> DeferRuntimeFn<T, Twhen, F, Fwhen>
where
	F: FnOnce(T),
	Fwhen: FnOnce(Twhen) -> bool
{
	#[inline]
	pub fn new(value: T, f: F, should_run_value: Twhen, should_run: Fwhen) -> Self {
		Self::_new(value, f, when::RuntimeFn {
			value: ManuallyDrop::new(should_run_value),
			f: ManuallyDrop::new(should_run)
		})
	}
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
	#[inline]
	fn drop(&mut self) {
		unsafe {
			let value = ManuallyDrop::take(&mut self.value);
			let when = ManuallyDrop::take(&mut self.when);
			let f = ManuallyDrop::take(&mut self.f);

			if !when.should_run() { return }
			f(value);
		}
	}
}

impl<T, W, F> Debug for Defer<T, W, F>
where
	T: Debug,
	W: when::When + Debug,
	F: FnOnce(T)
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Defer")
			.field("value", &**self)
			.field("when", &*self.when)
			.finish_non_exhaustive()
	}
}

impl<T, W, F> Display for Defer<T, W, F>
where
	T: Display,
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T, W, F, U> AsRef<U> for Defer<T, W, F>
where
	T: AsRef<U>,
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
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
	#[inline]
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
		DeferAlways::new(self, f)
	}

	#[inline]
	fn on_success_drop<F>(self, f: F) -> DeferSuccess<Self, F>
	where
		F: FnOnce(Self)
	{
		DeferSuccess::new(self, f)
	}

	#[inline]
	fn on_unwind_drop<F>(self, f: F) -> DeferUnwind<Self, F>
	where
		F: FnOnce(Self)
	{
		DeferUnwind::new(self, f)
	}
}

impl<T> OnDrop for T {}

#[cfg(test)]
mod tests {
	use super::*;
	use std::cell::Cell;
	use std::panic::{ AssertUnwindSafe, catch_unwind };

	#[test]
	fn defer_always_success() {
		let cell = Cell::new("");

		{
			defer!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
		}

		assert_eq!(cell.get(), "glory is cute");
	}

	#[test]
	fn defer_always_unwind() {
		let cell = Cell::new("");

		let _ = catch_unwind(AssertUnwindSafe(|| {
			defer!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
			panic!("panick a");
		}));

		assert_eq!(cell.get(), "glory is cute");
	}

	#[test]
	fn defer_success_success() {
		let cell = Cell::new("");

		{
			defer_success!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
		}

		assert_eq!(cell.get(), "glory is cute");
	}

	#[test]
	fn defer_success_unwind() {
		let cell = Cell::new("");

		let _ = catch_unwind(AssertUnwindSafe(|| {
			defer_success!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
			panic!("panick a");
		}));

		assert_eq!(cell.get(), "");
	}

	#[test]
	fn defer_unwind_success() {
		let cell = Cell::new("");

		{
			defer_unwind!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
		}

		assert_eq!(cell.get(), "");
	}

	#[test]
	fn defer_unwind_unwind() {
		let cell = Cell::new("");

		let _ = catch_unwind(AssertUnwindSafe(|| {
			defer_unwind!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
			panic!("panick a");
		}));

		assert_eq!(cell.get(), "glory is cute");
	}
}
