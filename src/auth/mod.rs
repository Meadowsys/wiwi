use crate::rand::{
	ThreadLocalChaCha8Rng as RngFast,
	ThreadLocalChaCha20Rng as RngSecure
};

mod error;
pub mod prim;

pub use error::{ Error, Result };
