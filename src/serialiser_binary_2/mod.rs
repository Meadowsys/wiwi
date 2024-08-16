mod bool;
mod value;

pub use self::value::{ Value, ValueBorrowed };

#[inline]
pub fn serialise_into<'h, T, O>(value: &'h T, buf: &mut O)
where
	T: ?Sized + Serialise,
	O: Output<'h>
{
	let serialiser = value.build_serialiser();

	buf.reserve(serialiser.estimated_capacity());
	buf.reserve_intern(serialiser.estimated_intern_elements());

	serialiser.serialise(buf);
}

pub trait Serialise {
	type Serialiser<'h>: Serialiser<'h>
	where
		Self: 'h;

	fn build_serialiser(&self) -> Self::Serialiser<'_>;
}

pub trait Serialiser<'h> {
	/// Get estimated capacity in bytes needed to serialise this item
	#[inline]
	fn estimated_capacity(&self) -> usize {
		0
	}

	/// Get estimated number of elements that will be interned
	#[inline]
	fn estimated_intern_elements(&self) -> usize {
		0
	}

	/// Serialise this element into the provided output buffer
	fn serialise<O>(&self, buf: &mut O)
	where
		O: Output<'h>;
}

pub trait Deserialise<'h> {}

pub trait Output<'h>: Sized {
	#[inline]
	fn reserve(&mut self, amount: usize) {
		let _ = amount;
	}

	#[inline]
	fn reserve_intern(&mut self, amount: usize) {
		let _ = amount;
	}

	fn intern<T>(&mut self, val: &T)
	where
		T: ?Sized + Internable<'h>;

	fn write_bytes(&mut self, bytes: &[u8]);

	#[inline]
	fn write_byte(&mut self, byte: u8) {
		self.write_bytes(&[byte])
	}
}

// pub trait Input<'h> {
// 	fn read_bytes(&mut self, amount: usize) -> Option<&'h [u8]>;
//
// 	#[inline]
// 	fn read_bytes_const<const N: usize>(&mut self) -> Option<&'h [u8; N]>
// 	where
// 		Self: Sized
// 	{
// 		Some(use_some!(
// 			self.read_bytes(N),
// 			b => unsafe { &*b.as_ptr().cast::<[u8; N]>() }
// 		))
// 	}
//
// 	#[inline]
// 	fn read_byte(&mut self) -> Option<u8> {
// 		Some(use_some!(
// 			self.read_bytes(1),
// 			bytes => unsafe { *bytes.as_ptr() }
// 		))
// 	}
// }

pub trait Internable<'h>
where
	Self: 'h
{
	fn to_borrowed_value(&self) -> ValueBorrowed<'h>;
}

impl<'h> Internable<'h> for bool {
	#[inline]
	fn to_borrowed_value(&self) -> ValueBorrowed<'h> {
		ValueBorrowed::Owned(Value::Bool(*self))
	}
}

macro_rules! impl_internable_num {
	{ $($(#[$meta:meta])* $int:ident $owned_variant:ident)* } => {
		$(
			$(#[$meta])*
			impl<'h> Internable<'h> for $int {
				#[inline]
				fn to_borrowed_value(&self) -> ValueBorrowed<'h> {
					ValueBorrowed::Owned(Value::$owned_variant(*self as _))
				}
			}
		)*
	}
}

impl_internable_num! {
	u8 IntUnsigned
	u16 IntUnsigned
	u32 IntUnsigned
	u64 IntUnsigned
	u128 IntUnsigned

	i8 IntSigned
	i16 IntSigned
	i32 IntSigned
	i64 IntSigned
	i128 IntSigned

	f32 F32
	f64 F64
}

// macro_rules! use_ok {
// 	($result:expr) => {
// 		match $result {
// 			Ok(val) => { val }
// 			Err(err) => { return Err(err) }
// 		}
// 	};
//
// 	($result:expr, #err $err:ident => $err_op:expr) => {
// 		match $result {
// 			Ok(val) => { val }
// 			Err($err) => { return $err_op }
// 		}
// 	};
//
// 	($result:expr, $val:ident => $op:expr) => {
// 		match $result {
// 			Ok($val) => { $op }
// 			Err(err) => { return Err(err) }
// 		}
// 	};
//
// 	($result:expr, $val:ident => $op:expr, #err $err:ident => $err_op:expr) => {
// 		match $result {
// 			Ok($val) => { $op }
// 			Err($err) => { return $err_op }
// 		}
// 	};
// }
// use use_ok;

// macro_rules! use_some {
// 	($option:expr) => {
// 		match $option {
// 			Some(val) => { val }
// 			None => { return None }
// 		}
// 	};
//
// 	($option:expr, #none => $none_op:expr) => {
// 		match $option {
// 			Some(val) => { val }
// 			None => { return $none_op }
// 		}
// 	};
//
// 	($option:expr, $val:ident => $op:expr) => {
// 		match $option {
// 			Some($val) => { $op }
// 			None => { return None }
// 		}
// 	};
//
// 	($option:expr, $val:ident => $op:expr, #none => $none_op:expr) => {
// 		match $option {
// 			Some($val) => { $op }
// 			None => { return $none_op }
// 		}
// 	};
// }
// use use_some;
