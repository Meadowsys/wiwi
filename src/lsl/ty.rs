use std::fmt;

#[derive(Clone, Copy)]
pub struct Float;
#[derive(Clone, Copy)]
pub struct Integer;
#[derive(Clone, Copy)]
pub struct Key;
#[derive(Clone, Copy)]
pub struct List;
#[derive(Clone, Copy)]
pub struct Rotation;
#[derive(Clone, Copy)]
pub struct String;
#[derive(Clone, Copy)]
pub struct Vector;
#[derive(Clone, Copy)]
pub struct Boolean;
#[derive(Clone, Copy)]
pub struct Quaternion;

pub trait Type {
	fn type_keyword(&self) -> &'static str;
}

macro_rules! impl_type_trait {
	($($type:ident $type_lsl:literal)*) => {
		$(
			impl Type for $type {
				fn type_keyword(&self) -> &'static str {
					$type_lsl
				}
			}
		)*
	}
}

impl_type_trait! {
	Float "float"
	Integer "integer"
	Key "key"
	List "list"
	Rotation "rotation"
	String "string"
	Vector "vector"
	Boolean "integer" // ?????
	Quaternion "quaternion"
}
