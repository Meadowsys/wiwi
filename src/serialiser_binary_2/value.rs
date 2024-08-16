use std::collections::BTreeMap;

pub enum Value {
	None,
	Bool(bool),
	IntUnsigned(u128),
	IntSigned(i128),
	// f16
	F32(f32),
	F64(f64),
	// f128
	// f256
	String(String),
	Array(Vec<Value>),
	Record(BTreeMap<String, Value>),
	Map(BTreeMap<Value, Value>),
	Binary(Vec<u8>)
}

pub enum ValueBorrowed<'h> {
	Owned(Value),
	BorrowedValue(&'h Value)
}

// pub enum ValueBorrowed<'h> {
// 	None,
// 	Bool(bool),
// 	IntUnsigned(u128),
// 	IntSigned(i128),
// 	// f16
// 	F32(f32),
// 	F64(f64),
// 	// f128
// 	// f256
// 	String(StringBorrowed<'h>),
// 	Array(ArrayBorrowed<'h>),
// 	// Record(BTreeMap<String, Value>),
// 	// Map(BTreeMap<Value, Value>),
// 	// Binary(Vec<u8>)
// }
//
// pub enum StringBorrowed<'h> {
// 	Owned(String),
// 	// BorrowedString(&'h String),
// 	BorrowedStr(&'h str)
// }
//
// pub enum ArrayBorrowed<'h> {
// 	Owned(Vec<Value>),
// 	OwnedValueBorrowed(Vec<ValueBorrowed<'h>>),
// 	BorrowedSliceValue(&'h [Value]),
// 	BorrowedSliceValueBorrowed(&'h [ValueBorrowed<'h>]),
//
// 	// BorrowedVecValue(&'h Vec<Value>),
// 	// BorrowedVecValueBorrowed(&'h Vec<ValueBorrowed<'h>>),
// }
//
// pub enum RecordBorrowed<'h> {
// 	// Owned(BTreeMap<>)
// }
