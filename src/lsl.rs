#![allow(
	dead_code,
	reason = "wip"
)]
#![allow(
	clippy::missing_inline_in_public_items,
	reason = "not performance sensitive and/or wip"
)]

extern crate hashbrown;
extern crate wiwiwiwiwiwiwiwiwiwi;

use crate::prelude::*;
use crate::chain::*;
use crate::rc::RcThread;
use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
use ident::{ Ident, IdentIncrementer };

pub use wiwiwiwiwiwiwiwiwiwi::state;

mod ident;

// fn script() {}

/// LSL type
trait Type {
	#[doc(hidden)]
	fn __assert_obj_safe(_: &dyn Type, _: private::SealedStruct)
	where
		Self: Sized;

	fn keyword() -> &'static str
	where
		Self: Sized;

	fn dyn_keyword(&self) -> &'static str;
}

/// Rust type that can be used as initialisation values for LSL types
trait TypeInit<T>
where
	T: Type
{
	#[doc(hidden)]
	fn __assert_obj_safe(_: &dyn TypeInit<T>, _: private::SealedStruct)
	where
		Self: Sized;

	// fn into_type_init_dyn(self) -> Box<dyn TypeInitDyn>;
}

/// LSL type that a Rust type should be inferred to represent
trait TypeInitDefault
where
	Self: TypeInit<Self::Type>,
	Self::Type: Type
{
	type Type;
}

// trait TypeInitDyn {
// 	#[doc(hidden)]
// 	fn __assert_obj_safe(_: &dyn TypeInitDyn, _: private::SealedStruct)
// 	where
// 		Self: Sized;
// }

/// Strongly typed LSL variable identifier
struct Var<T>
where
	T: Type
{
	id: Ident,
	__marker: PhantomData<fn(T) -> T>
}

/// Dynamically typed LSL variable identifier
struct VarDyn {
	id: Ident,
	ty: Box<dyn Type>
}

/// "Strongly" typed LSL value initialisers, which is `dyn` and can
/// be another variable or a Rust value
///
/// The strong typing is on the LSL value, not the type that is used
/// to initialise it.
struct VarInit<T>
where
	T: Type
{
	init: Box<dyn TypeInit<T>>
}

impl<T> VarInit<T>
where
	T: Type
{
	fn new<I>(init: I) -> Self
	where
		I: TypeInitDefault<Type = T> + 'static
	{
		Self { init: Box::new(init) }
	}
}




























































// // fn script(f: impl FnOnce(&mut ScriptCx)) -> Script {
// // 	Script::new()
// // 		.into_generic_chain()
// // 		.with_inner(|s| f(&mut s.cx()))
// // 		.into_inner()
// // }

// struct Script {
// 	// vars: Vec<(VarDyn, VarInit)>
// 	// default_state: State,
// 	// states: HashMap<&'static str, State>,
// 	// id_incrementer: IdentIncrementer
// }

// struct ScriptCx<'h> {
// 	inner: &'h mut Script
// }

// impl Script {}

// struct Vars {
// 	vars: Vec<(VarDyn, InitialiserDyn)>
// }

// // impl Script {
// // 	fn new() -> Self {
// // 		Self {
// // 			vars: Vec::new(),
// // 			default_state: State::new(),
// // 			states: HashMap::new(),
// // 			id_incrementer: IdentIncrementer::new()
// // 		}
// // 	}

// // 	fn cx(&mut self) -> ScriptCx {
// // 		ScriptCx { inner: self }
// // 	}
// // }

// // impl ScriptCx<'_> {
// // 	fn var<T>(&mut self, val: T) -> &mut Self
// // 	where
// // 		T: DefaultType
// // 	{
// // 		todo!()
// // 	}

// // 	fn ll_state_default(&mut self, f: impl FnOnce(&mut StateCx)) -> &mut Self {
// // 		f(&mut self.inner.default_state.cx(self.inner.id_incrementer.clone()));
// // 		self
// // 	}

// // 	fn ll_state(&mut self, state: &'static str, f: impl FnOnce(&mut StateCx)) -> &mut Self {
// // 		f(&mut self.inner.states.entry(state).or_insert_with(State::new).cx(self.inner.id_incrementer.clone()));
// // 		self
// // 	}
// // }

// // struct State {
// // 	events: HashMap<&'static str, Function>
// // }

// // impl State {
// // 	fn new() -> Self {
// // 		Self {
// // 			events: HashMap::new()
// // 		}
// // 	}

// // 	fn cx(&mut self, id_incrementer: IdentIncrementer) -> StateCx {
// // 		StateCx { inner: self, id_incrementer }
// // 	}
// // }

// // struct StateCx<'h> {
// // 	inner: &'h mut State,
// // 	id_incrementer: IdentIncrementer
// // }

// // impl StateCx<'_> {
// // 	fn ll_ev_state_entry(&mut self, f: impl FnOnce(&mut FunctionCx)) -> &mut Self {
// // 		// ???
// // 		self.ll_try_ev_state_entry(f).expect("event `state_entry` already registered")
// // 	}

// // 	fn ll_try_ev_state_entry(&mut self, f: impl FnOnce(&mut FunctionCx)) -> Option<&mut Self> {
// // 		if let Entry::Vacant(entry) = self.inner.events.entry("state_entry") {
// // 			f(&mut entry.insert(Function::new([], TypeDyn::unit())).cx(self.id_incrementer.clone()));
// // 			Some(self)
// // 		} else {
// // 			None
// // 		}
// // 	}
// // }

// // struct Function {
// // 	args: Vec<VarDyn>,
// // 	ret: TypeDyn,
// // 	statements: Vec<Statement>
// // }

// // impl Function {
// // 	fn new(args: impl IntoIterator<Item = VarDyn>, ret: TypeDyn) -> Self {
// // 		Self {
// // 			args: args.into_iter().collect(),
// // 			ret,
// // 			statements: Vec::new()
// // 		}
// // 	}

// // 	fn cx(&mut self, id_incrementer: IdentIncrementer) -> FunctionCx {
// // 		FunctionCx { inner: self, id_incrementer }
// // 	}
// // }

// // struct FunctionCx<'h> {
// // 	inner: &'h mut Function,
// // 	id_incrementer: IdentIncrementer
// // }

// // impl FunctionCx<'_> {
// // 	fn ll_say(&mut self, channel: i32, msg: &str) {
// // 		// self.inner
// // 		// ??
// // 	}
// // }

// // struct Statement {}

// // fn __assert_type_dyn_compat(_: &dyn Type) {}

// // macro_rules! impl_type {
// // 	{ $($(#[$meta:meta])* $struct:ident $keyword:literal)* } => {
// // 		$(
// // 			$(#[$meta])*
// // 			#[expect(non_camel_case_types, reason = "lsl primitive")]
// // 			struct $struct { __private: () }

// // 			impl $struct {
// // 				fn new() -> Self {
// // 					Self { __private: () }
// // 				}
// // 			}

// // 			impl Type for $struct {
// // 				fn keyword() -> &'static str
// // 				where
// // 					Self: Sized
// // 				{
// // 					$keyword
// // 				}

// // 				fn dyn_keyword(&self) -> &'static str {
// // 					$keyword
// // 				}
// // 			}

// // 			impl private::Sealed for $struct {}
// // 		)*
// // 	}
// // }

// // impl_type! {
// // 	/// LSL integer (signed 32-bit integer)
// // 	integer "integer"

// // 	/// LSL floating-point number (32-bit floating-point number)
// // 	float "float"

// // 	/// LSL string (textual data, internally UTF-16 in mono)
// // 	string "string"

// // 	/// LSL key (UUID)
// // 	key "key"

// // 	/// LSL vector (set of 3 floating-point numbers)
// // 	vector "vector"

// // 	/// LSL rotation (set of 4 floating-point numbers)
// // 	rotation "rotation"

// // 	/// LSL list (list of any other LSL value)
// // 	list "integer"

// // 	/// Boolean value (represented underneath by LSL integer)
// // 	boolean "integer"

// // 	/// Unit type (doesn't actually exist in LSL)
// // 	unit ""
// // }

// // struct TypeDyn {
// // 	inner: Box<dyn Type>
// // }

// // macro_rules! impl_type_dyn {
// // 	{ $($type:ident)*} => {
// // 		$(
// // 			fn $type() -> Self {
// // 				Self { inner: Box::new($type::new()) }
// // 			}
// // 		)*
// // 	}
// // }

// // impl TypeDyn {
// // 	impl_type_dyn! {
// // 		integer
// // 		float
// // 		string
// // 		key
// // 		vector
// // 		rotation
// // 		list
// // 		boolean
// // 		unit
// // 	}
// // }

// // struct Var<T>
// // where
// // 	T: Type
// // {
// // 	id: Ident,
// // 	__marker: PhantomData<fn(T) -> T>
// // }

// // impl<T> Var<T>
// // where
// // 	T: Type
// // {
// // 	fn with_id(id: Ident) -> Self {
// // 		Self { id, __marker: PhantomData }
// // 	}
// // }

// // struct VarDyn {
// // 	id: Ident,
// // 	ty: TypeDyn
// // }

// // macro_rules! impl_var_dyn {
// // 	{ $($type:ident)*} => {
// // 		$(
// // 			fn $type(id: Ident) -> Self {
// // 				Self { id, ty: TypeDyn::$type() }
// // 			}
// // 		)*
// // 	}
// // }

// // impl VarDyn {
// // 	impl_var_dyn! {
// // 		integer
// // 		float
// // 		string
// // 		key
// // 		vector
// // 		rotation
// // 		list
// // 		boolean
// // 	}
// // }

// // struct VarInit {}

// // struct IdentIncrementer {
// // 	next: RcThread<cell::Cell<Ident>>
// // }

// // impl IdentIncrementer {
// // 	fn new() -> Self {
// // 		Self { next: RcThread::from_value(cell::Cell::new(0)) }
// // 	}

// // 	fn next(&self) -> Ident {
// // 		let next = self.next.as_value_ref().get();
// // 		self.next.as_value_ref().set(next.checked_add(1).unwrap());
// // 		next
// // 	}
// // }

// // impl Clone for IdentIncrementer {
// // 	fn clone(&self) -> Self {
// // 		Self { next: RcThread::clone(&self.next) }
// // 	}
// // }

// // /// notouchie
// // mod private {
// // 	/// notouchie
// // 	trait Sealed {}
// // }

// // fn _test() {
// // 	use crate::lsl;

// // 	lsl::script(|cx| {
// // 		cx.ll_state_default(|cx| {
// // 			cx.ll_ev_state_entry(|cx| {
// // 			});
// // 		});
// // 	});
// // }

// // thread_local! {
// // 	static CONTEXT: cell::RefCell<Vec<Box<dyn Any>>> = cell::RefCell::new(Vec::new());
// // }

// // struct ScriptCx {}
// // struct StateCx {}
// // struct FunctionCx {}

// // fn with_script_cx(f: impl FnOnce(&mut ScriptCx)) {
// // 	CONTEXT.with_borrow_mut(|cx| f(cx.last_mut().unwrap().downcast_mut::<ScriptCx>().unwrap()))
// // }

// // fn with_state_cx(f: impl FnOnce(&mut StateCx)) {
// // 	CONTEXT.with_borrow_mut(|cx| f(cx.last_mut().unwrap().downcast_mut::<StateCx>().unwrap()))
// // }

// // fn with_function_cx(f: impl FnOnce(&mut FunctionCx)) {
// // 	CONTEXT.with_borrow_mut(|cx| f(cx.last_mut().unwrap().downcast_mut::<FunctionCx>().unwrap()))
// // }

mod private {
	pub struct SealedStruct;
}
