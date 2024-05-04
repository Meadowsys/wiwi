use rand::{ Rng, rngs::OsRng };

pub struct Salt {
	salt: [u8; 32]
}

impl Salt {
	pub(in crate::auth) fn generate() -> Self {
		let mut salt = [0u8; 32];
		OsRng.fill(&mut salt);
		Self { salt }
	}
}
