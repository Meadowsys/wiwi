use crate::z85::{ encode_z85, decode_z85 };
use super::*;
use std::{ fmt, str };

/// 32-byte salt
pub struct Salt {
	pub(in crate::auth) inner: [u8; 32]
}

impl Salt {
	pub(in crate::auth) fn generate() -> Self {
		Self { inner: util::rand_array() }
	}
}

// impl fmt::Display for Salt {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		f.write_str(&encode_z85(&self.salt))
// 	}
// }

// impl str::FromStr for Salt {
// 	type Err = error::Error;
// 	fn from_str(s: &str) -> Result<Self, error::Error> {
// 		let salt = (&*decode_z85(s.as_bytes())?).try_into()?;
// 		Ok(Salt { salt })
// 	}
// }
