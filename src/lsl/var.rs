use super::*;
use hashbrown::HashMap;
use std::cell::RefCell;
use std::convert::identity;

thread_local! {
	pub(super) static VAR_INIT_VALUES: RefCell<HashMap<u64, String>> = {
		RefCell::new(HashMap::with_capacity(16))
	}
}

#[derive(Clone, Copy)]
pub struct Var<T> {
	id: u64,
	ty: T,
	has_init: bool,
	_st: SingleThreadMarker
}

fn var_decl<T>(ty: T, init: Option<String>) -> Var<T>
where
	T: ty::Type + Copy + 'static
{
	let id = util::next_id();
	let _st = PhantomData;
	let has_init = if let Some(init) = init {
		VAR_INIT_VALUES.with_borrow_mut(|init_val_map| {
			let prev = init_val_map.insert(id, init);
			assert!(prev.is_none())
		});
		true
	} else { false };

	let var = Var { id, ty, has_init, _st };

	ctx::CONTEXT.with_borrow_mut(|ctx| {
		ctx.last_mut()
			.expect("cannot declare variable outside a script")
			.borrow_var_delarable()
			.declare_ident(Box::new(var));
	});

	var
}

impl<T: ty::Type> ty::Type for Var<T> {
	fn type_keyword(&self) -> &'static str {
		T::type_keyword(&self.ty)
	}
}

pub(super) trait VarTrait {
	fn get_ty(&self) -> &dyn ty::Type;
	fn id(&self) -> u64;
	fn has_init(&self) -> bool;
}

impl<T: ty::Type> VarTrait for Var<T> {
	fn get_ty(&self) -> &dyn ty::Type {
		&self.ty
	}
	fn id(&self) -> u64 {
		self.id
	}
	fn has_init(&self) -> bool {
		self.has_init
	}
}

pub(super) trait VarDeclarable {
	fn declare_ident(&mut self, ident: Box<dyn VarTrait>);
}

impl VarDeclarable for script::Script {
	fn declare_ident(&mut self, ident: Box<dyn VarTrait>) {
		let prev = self.global_vars.insert(ident.id(), ident);
		assert!(prev.is_none());
	}
}

// with initialiser

pub fn v_f32<I: InitValue<ty::Float>>(init: I) -> Var<ty::Float> {
	var_decl(ty::Float, Some(init.into_init_value()))
}

pub fn v_i64<I: InitValue<ty::Integer>>(init: I) -> Var<ty::Integer> {
	var_decl(ty::Integer, Some(init.into_init_value()))
}

pub fn v_key<I: InitValue<ty::Key>>(init: I) -> Var<ty::Key> {
	var_decl(ty::Key, Some(init.into_init_value()))
}

pub fn v_lst<I: InitValue<ty::List>>(init: I) -> Var<ty::List> {
	var_decl(ty::List, Some(init.into_init_value()))
}

pub fn v_rot<I: InitValue<ty::Rotation>>(init: I) -> Var<ty::Rotation> {
	var_decl(ty::Rotation, Some(init.into_init_value()))
}

pub fn v_str<I: InitValue<ty::String>>(init: I) -> Var<ty::String> {
	var_decl(ty::String, Some(init.into_init_value()))
}

pub fn v_vec<I: InitValue<ty::Vector>>(init: I) -> Var<ty::Vector> {
	var_decl(ty::Vector, Some(init.into_init_value()))
}

pub fn v_bol<I: InitValue<ty::Boolean>>(init: I) -> Var<ty::Boolean> {
	var_decl(ty::Boolean, Some(init.into_init_value()))
}

pub fn v_qua<I: InitValue<ty::Quaternion>>(init: I) -> Var<ty::Quaternion> {
	var_decl(ty::Quaternion, Some(init.into_init_value()))
}

// without initialiser

pub fn v_f32_uninit() -> Var<ty::Float> {
	var_decl(ty::Float, None)
}

pub fn v_i64_uninit() -> Var<ty::Integer> {
	var_decl(ty::Integer, None)
}

pub fn v_key_uninit() -> Var<ty::Key> {
	var_decl(ty::Key, None)
}

pub fn v_lst_uninit() -> Var<ty::List> {
	var_decl(ty::List, None)
}

pub fn v_rot_uninit() -> Var<ty::Rotation> {
	var_decl(ty::Rotation, None)
}

pub fn v_str_uninit() -> Var<ty::String> {
	var_decl(ty::String, None)
}

pub fn v_vec_uninit() -> Var<ty::Vector> {
	var_decl(ty::Vector, None)
}

pub fn v_bol_uninit() -> Var<ty::Boolean> {
	var_decl(ty::Boolean, None)
}

pub fn v_qua_uninit() -> Var<ty::Quaternion> {
	var_decl(ty::Quaternion, None)
}

pub trait InitValue<T> {
	fn into_init_value(self) -> String;
}

macro_rules! impl_init_value {
	($($ty:ty, $target:ty, $closure:expr;)*) => {
		$(
			impl InitValue<$target> for $ty {
				fn into_init_value(self) -> String {
					#[inline(always)]
					fn call(item: $ty, f: impl FnOnce($ty) -> String) -> String {
						// ...sure
						f(item)
					}

					call(self, $closure)
				}
			}
		)*
	}
}

fn empty_string(_: ()) -> String {
	String::new()
}

impl_init_value! {
	// float
	(), ty::Float, empty_string;
	f32, ty::Float, |f| format!("={f}");

	// int
	(), ty::Integer, empty_string;
	u8, ty::Integer, |i| format!("={i}");
	u16, ty::Integer, |i| format!("={i}");
	u32, ty::Integer, |i| format!("={i}");
	i8, ty::Integer, |i| format!("={i}");
	i16, ty::Integer, |i| format!("={i}");
	i32, ty::Integer, |i| format!("={i}");
	i64, ty::Integer, |i| format!("={i}");

	// key (for now, we're not gonna check its valid?)
	(), ty::Key, empty_string;
	&str, ty::Key, |s| format!("={s:?}");
	String, ty::Key, |s| format!("={s:?}");

	// list
	(), ty::List, empty_string;

	// rotation
	(), ty::Rotation, empty_string;

	// string
	(), ty::String, empty_string;
	&str, ty::String, |s| format!("={s:?}");
	String, ty::String, |s| format!("={s:?}");

	// vector
	(), ty::Vector, empty_string;
	(f32, f32, f32), ty::Vector, |(f1, f2, f3)| format!("=<{f1}, {f2}, {f3}>");

	// boolean
	(), ty::Boolean, empty_string;
	bool, ty::Boolean, |b| format!("={}", b as usize);

	// quaternion
	(), ty::Quaternion, empty_string;
}
