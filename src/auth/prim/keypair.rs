use crate::auth::{ Result, RngSecure };
use p384::ecdsa::signature::{ RandomizedSigner as _, Verifier as _ };

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

impl Keypair {
	#[inline]
	pub fn generate() -> Self {
		let secret_key = p384::SecretKey::random(&mut RngSecure);
		let public_key = secret_key.public_key();

		let public_key = PublicKey { inner: public_key };
		let secret_key = SecretKey { inner: secret_key };

		Self { public_key, secret_key }
	}

	#[inline]
	pub fn from_secret_key(secret_key: SecretKey) -> Self {
		let public_key = secret_key.inner.public_key();
		let public_key = PublicKey { inner: public_key };
		Self { public_key, secret_key }
	}

	#[inline]
	pub fn into_inner(self) -> (PublicKey, SecretKey) {
		let Self { public_key, secret_key } = self;
		(public_key, secret_key)
	}

	#[inline]
	pub fn sign(&self, bytes: &[u8]) -> Signature {
		self.secret_key.sign(bytes)
	}

	#[inline]
	pub fn verify(&self, bytes: &[u8], signature: &Signature) -> bool {
		self.public_key.verify(bytes, signature)
	}
}

impl PublicKey {
	#[inline]
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		let key = p384::PublicKey::from_sec1_bytes(bytes)?;
		Ok(Self { inner: key })
	}

	#[inline]
	pub fn to_bytes(&self) -> Vec<u8> {
		self.inner.to_sec1_bytes().into()
	}

	#[inline]
	pub fn verify(&self, bytes: &[u8], signature: &Signature) -> bool {
		let key = p384::ecdsa::VerifyingKey::from(&self.inner);
		key.verify(bytes, &signature.inner).is_ok()
	}
}

impl SecretKey {
	#[inline]
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		let key = p384::SecretKey::from_sec1_der(bytes)?;
		Ok(SecretKey { inner: key })
	}

	#[inline]
	pub fn to_bytes(&self) -> Result<Vec<u8>> {
		let key = self.inner.to_sec1_der()?;
		Ok((**key).into())
	}

	#[inline]
	pub fn sign(&self, bytes: &[u8]) -> Signature {
		let key = p384::ecdsa::SigningKey::from(&self.inner);
		let signature = key.sign_with_rng(&mut RngSecure, bytes);
		Signature { inner: signature }
	}
}

impl Signature {
	#[inline]
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		let signature = p384::ecdsa::Signature::from_slice(bytes)?;
		Ok(Self { inner: signature })
	}

	#[inline]
	pub fn to_bytes(&self) -> Vec<u8> {
		self.inner.to_vec()
	}

	#[inline]
	pub fn verify(&self, bytes: &[u8], public_key: &PublicKey) -> bool {
		public_key.verify(bytes, self)
	}
}
