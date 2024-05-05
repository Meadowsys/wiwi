use super::*;
use chacha20poly1305::{ aead::Aead, KeyInit, XChaCha20Poly1305 };

/// Symmetric encryption using XChaCha20Poly1305
pub struct ChaCha {
	pub(in crate::auth) inner: Vec<u8>
}

impl ChaCha {
	pub(in crate::auth) fn encrypt(
		bytes: &[u8],
		key: &Key,
		nonce: &Nonce
	) -> Result<Self> {
		let key = chacha20poly1305::Key::from(key.inner);
		let cipher = XChaCha20Poly1305::new(&key);

		let encrypted = cipher.encrypt(&nonce.inner, bytes)?;
		Ok(Self { inner: encrypted })
	}
}

pub struct Nonce {
	pub(in crate::auth) inner: chacha20poly1305::XNonce
}

impl Nonce {
	pub(in crate::auth) fn generate() -> Self {
		let inner = chacha20poly1305::XNonce::from(util::rand_array());
		Self { inner }
	}
}

pub struct Key {
	pub(in crate::auth) inner: chacha20poly1305::Key
}

impl Key {
	pub(in crate::auth) fn generate() -> Self {
		let inner = chacha20poly1305::Key::from(util::rand_array());
		Self { inner }
	}

	pub(in crate::auth) fn from_bytes(key: &[u8; 32]) -> Self {
		let inner = chacha20poly1305::Key::from(*key);
		Self { inner }
	}
}
