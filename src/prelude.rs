//! Prelude module, exporting commonly-used items

pub use crate::option::{
	Option,
	Option::Some,
	Option::None
};
pub use crate::phantom::{
	PhantomData,
	PhantomPinned
};
pub use crate::result::{
	Result,
	Result::Ok,
	Result::Err
};
