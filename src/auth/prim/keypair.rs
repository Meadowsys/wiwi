use super::*;
use rand::thread_rng;

/// P384 keypairs for signing
pub struct Keypair {
	pub(in crate::auth) public_key: PublicKey,
	pub(in crate::auth) secret_key: SecretKey
}

pub struct PublicKey {
	pub(in crate::auth) inner: p384::PublicKey
}

pub struct SecretKey {
	pub(in crate::auth) inner: p384::SecretKey
}

impl Keypair {
	pub(in crate::auth) fn generate() -> Self {
		let secret_key = p384::SecretKey::random(&mut thread_rng());
		let public_key = secret_key.public_key();

		let public_key = PublicKey { inner: public_key };
		let secret_key = SecretKey { inner: secret_key };

		Self { public_key, secret_key }
	}
}

impl PublicKey {
	pub(in crate::auth) fn to_bytes(&self) -> Vec<u8> {
		self.inner.to_sec1_bytes().into()
	}
}

impl SecretKey {
	pub(in crate::auth) fn to_bytes(&self) -> Result<Vec<u8>> {
		let key = self.inner.to_sec1_der()?;
		Ok((**key).into())
	}
}
