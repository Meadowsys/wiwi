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

pub struct Signature {
	inner: p384::ecdsa::Signature
}

pub fn generate() -> Keypair {
	let secret_key = p384::SecretKey::random(&mut thread_rng());
	let public_key = secret_key.public_key();

	let public_key = PublicKey { inner: public_key };
	let secret_key = SecretKey { inner: secret_key };
	Keypair { public_key, secret_key }
}

pub fn sec_key_from_bytes(bytes: &[u8]) -> Result<SecretKey> {
	let key = p384::SecretKey::from_sec1_der(bytes)?;
	Ok(SecretKey { inner: key })
}

impl Keypair {
	pub fn public_key(&self) -> &PublicKey {
		&self.public_key
	}

	pub fn secret_key(&self) -> &SecretKey {
		&self.secret_key
	}

	pub(crate) fn into_inner(self) -> (PublicKey, SecretKey) {
		let Self { public_key, secret_key } = self;
		(public_key, secret_key)
	}
}

impl PublicKey {
	pub fn to_bytes(&self) -> Vec<u8> {
		self.inner.to_sec1_bytes().into()
	}

	pub(crate) fn verify(&self, bytes: &[u8], sig: &Signature) -> bool {
		use p384::ecdsa::signature::Verifier;
		let key = p384::ecdsa::VerifyingKey::from(&self.inner);
		key.verify(bytes, &sig.inner).is_ok()
	}
}

impl SecretKey {
	pub fn to_bytes(&self) -> Result<Vec<u8>> {
		let key = self.inner.to_sec1_der()?;
		Ok((**key).into())
	}

	pub(crate) fn sign(&self, bytes: &[u8]) -> Signature {
		use p384::ecdsa::signature::RandomizedSigner as _;

		let key = p384::ecdsa::SigningKey::from(&self.inner);
		let sig = key.sign_with_rng(&mut thread_rng(), bytes);
		Signature { inner: sig }
	}
}
