use super::*;
use rand::thread_rng;

pub struct Keypair {
	public_key: PublicKey,
	secret_key: SecretKey
}

pub struct PublicKey {
	inner: p384::PublicKey
}

pub struct SecretKey {
	inner: p384::SecretKey
}

pub fn generate() -> Keypair {
	let secret_key = p384::SecretKey::random(&mut thread_rng());
	let public_key = secret_key.public_key();

	let public_key = PublicKey { inner: public_key };
	let secret_key = SecretKey { inner: secret_key };
	Keypair { public_key, secret_key }
}

impl Keypair {
	pub fn public_key(&self) -> &PublicKey {
		&self.public_key
	}

	pub fn secret_key(&self) -> &SecretKey {
		&self.secret_key
	}
}

impl PublicKey {
	pub fn to_bytes(&self) -> Vec<u8> {
		self.inner.to_sec1_bytes().into()
	}
}

impl SecretKey {
	pub fn to_bytes(&self) -> Result<Vec<u8>> {
		let key = self.inner.to_sec1_der()?;
		Ok((**key).into())
	}
}
