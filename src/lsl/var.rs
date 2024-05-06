use super::*;

use std::num::NonZeroU64;

#[derive(Clone, Copy)]
pub struct Var<T> {
	id: u64,
	val: Option<val::Val<T>>,
	ty: T,
	_st: SingleThreadMarker
}

fn var_decl_untyped<T>(ty: T, init: Option<String>) -> Var<T>
where
	T: ty::Type + Copy + 'static
{
	let id = util::next_id().get();
	let val = init.map(|init| val::store_untyped(ty, init));
	let _st = PhantomData;
	let var = Var { id, val, ty, _st };

	ctx::with(|ctx| {
		ctx.borrow_var_delarable()
			.declare_var(Box::new(var))
	});

	var
}

pub(super) trait VarTrait {
	fn get_ty(&self) -> &dyn ty::Type;
	fn id(&self) -> u64;
	fn val(&self) -> Option<&dyn val::ValTrait>;
}

impl<T: ty::Type> VarTrait for Var<T> {
	fn get_ty(&self) -> &dyn ty::Type {
		&self.ty
	}

	fn id(&self) -> u64 {
		self.id
	}

	fn val(&self) -> Option<&dyn val::ValTrait> {
		self.val.as_ref().map(|v| v as &dyn val::ValTrait)
	}
}

pub trait IntoVarInit<T> {
	fn to_var_init(&self) -> Option<String>;
}

impl<T: ty::Type> IntoVarInit<T> for Var<T> {
	fn to_var_init(&self) -> Option<String> {
		self.val.as_ref().map(|val| val.with(|val| val.into()))
	}
}

// this one conflicts with the one above. best I can do I think, is macro for
// specific types to use the IntoVal impl (below)
//
// impl<T: ty::Type, I> IntoVarInit<T> for I
// where
// 	I: val::IntoVal<T>
// {
// 	fn into_var_init(self) -> Option<String> {
// 		Some(self.into_value())
// 	}
// }

macro_rules! passthrough_into_var_init_impl {
	{ $(($ty:ty: $($bound_stuff:tt)*))* } => {
		$(
			impl<T: ty::Type> IntoVarInit<T> for $ty
			where
				$($bound_stuff)* : val::IntoVal<T>
			{
				fn to_var_init(&self) -> Option<String> {
					use val::IntoVal as _;
					Some(self.to_value())
				}
			}
		)*
	}
}

passthrough_into_var_init_impl! {
	(String: String)
	(&str: for<'h> &'h str)
	(i8: i8)
	(i16: i16)
	(i32: i32)
	(i64: i64)
	(u8: u8)
	(u16: u16)
	(u32: u32)
	(u64: u64)
}

macro_rules! var_init_fns {
	($($fn_name:ident, $fn_name_uninit:ident, $ty:path)*) => {
		$(
			pub fn $fn_name<I: IntoVarInit<$ty>>(init: I) -> Var<$ty> {
				var_decl_untyped($ty, init.to_var_init())
			}
			pub fn $fn_name_uninit() -> Var<$ty> {
				var_decl_untyped($ty, None)
			}
		)*
	}
}

var_init_fns! {
	var_float, var_float_uninit, ty::Float
	var_flo, var_flo_uninit, ty::Float

	var_integer, var_integer_uninit, ty::Integer
	var_int, var_int_uninit, ty::Integer

	var_key, var_key_uninit, ty::Key

	var_list, var_list_uninit, ty::List
	var_lst, var_lst_uninit, ty::List

	var_rotation, var_rotation_uninit, ty::Rotation
	var_rot, var_rot_uninit, ty::Rotation

	var_string, var_string_uninit, ty::String
	var_str, var_str_uninit, ty::String

	var_vector, var_vector_uninit, ty::Vector
	var_vec, var_vec_uninit, ty::Vector

	var_bool, var_bool_uninit, ty::Boolean
	var_bol, var_bol_uninit, ty::Boolean

	var_quaternion, var_quaternion_uninit, ty::Quaternion
	var_qua, var_qua_uninit, ty::Quaternion
}

macro_rules! impl_unit_init_values {
	($($ty:ty)*) => {
		$(
			impl IntoVarInit<$ty> for () {
				fn to_var_init(&self) -> Option<String> {
					None
				}
			}
		)*
	}
}

impl_unit_init_values! {
	ty::Float
	ty::Integer
	ty::Key
	ty::List
	ty::Rotation
	ty::String
	ty::Vector
	ty::Boolean
	ty::Quaternion
}
