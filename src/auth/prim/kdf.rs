use crate::auth::error::Error;
use crate::z85::{ encode_z85, decode_z85 };
use super::*;
use argon2::{ Algorithm, Argon2, PasswordHasher, Version };
use std::{ fmt, str };

const ALG: Algorithm = Algorithm::Argon2id;
const VERSION: Version = Version::V0x13;
const M_COST: u32 = 0x10000;
const T_COST: u32 = 4;
const P_COST: u32 = 2;

/// Key derivation function (argon2).
pub struct Kdf {
	alg: Algorithm,
	version: Version,
	m_cost: u32,
	t_cost: u32,
	p_cost: u32,
	bytes: [u8; 32]
}

impl Kdf {
	fn new() -> Self {
		Self {
			alg: ALG,
			version: VERSION,
			m_cost: M_COST,
			t_cost: T_COST,
			p_cost: P_COST,
			bytes: [0u8; 32]
		}
	}

	fn get_hasher(&self) -> Result<Argon2<'static>> {
		Ok(Argon2::new(
			self.alg,
			self.version,
			argon2::ParamsBuilder::new()
				.m_cost(self.m_cost)
				.t_cost(self.t_cost)
				.p_cost(self.p_cost)
				.build()?
		))
	}

	pub(in crate::auth) fn hash_and_salt(
		bytes: &[u8],
		salt: &salt::Salt
	) -> Result<Self> {
		let mut this = Self::new();
		this.get_hasher()?.hash_password_into(
			bytes,
			&salt.inner,
			&mut this.bytes
		)?;
		Ok(this)
	}

	pub(in crate::auth) fn as_bytes(&self) -> &[u8; 32] {
		&self.bytes
	}
}

// impl fmt::Display for Kdf {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		let Self { alg, version, m_cost, t_cost, p_cost, bytes } = self;
//
// 		let version = *version as u32;
// 		let bytes = encode_z85(bytes);
//
// 		// somewhat surprisingly, comma isn't used in z85
// 		write!(f, "{alg},{version},{m_cost},{t_cost},{p_cost},{bytes}")
// 	}
// }
//
// impl str::FromStr for Kdf {
// 	type Err = error::Error;
//
// 	fn from_str(s: &str) -> Result<Self, error::Error> {
// 		let mut iter = s.split(',');
//
// 		macro_rules! next {
// 			() => {
// 				iter.next().ok_or_else(|| error::Error::parse_kdf())?
// 			}
// 		}
//
// 		let alg = next!().parse()?;
// 		let version = next!().parse::<u32>()?.try_into()?;
// 		let m_cost = next!().parse()?;
// 		let t_cost = next!().parse()?;
// 		let p_cost = next!().parse()?;
// 		let bytes = (*decode_z85(next!().as_bytes())?).try_into()?;
//
// 		if iter.next().is_some() { return Err(Error::parse_kdf()) }
//
// 		Ok(Self { alg, version, m_cost, t_cost, p_cost, bytes })
// 	}
// }
