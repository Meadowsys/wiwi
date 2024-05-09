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

pub struct Kdf {
	alg: Algorithm,
	version: Version,
	m_cost: u32,
	t_cost: u32,
	p_cost: u32,
	output: [u8; 32]
}

fn get_hasher() -> Result<Argon2<'static>> {
	Ok(Argon2::new(
		ALG,
		VERSION,
		argon2::ParamsBuilder::new()
			.m_cost(M_COST)
			.t_cost(T_COST)
			.p_cost(P_COST)
			.build()?
	))
}

pub fn hash(
	bytes: &[u8],
	salt: &salt::Salt
) -> Result<Kdf> {
	let mut res = Kdf {
		alg: ALG,
		version: VERSION,
		m_cost: M_COST,
		t_cost: T_COST,
		p_cost: P_COST,
		output: util::rand_array()
	};

	get_hasher()?.hash_password_into(
		bytes,
		salt.as_bytes(),
		&mut res.output
	)?;

	Ok(res)
}

impl Kdf {
	pub fn as_hash_bytes(&self) -> &[u8; 32] {
		&self.output
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
