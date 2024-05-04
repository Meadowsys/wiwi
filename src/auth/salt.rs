use crate::z85::{ encode_z85, decode_z85 };
use super::*;
use rand::{ Rng, rngs::OsRng };
use std::{ fmt, str };

pub struct Salt {
	pub salt: [u8; 32]
}

impl Salt {
	pub(in crate::auth) fn generate() -> Self {
		let mut salt = [0u8; 32];
		OsRng.fill(&mut salt);
		Self { salt }
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
