use super::*;
use chacha20poly1305::{ aead::Aead, KeyInit, XChaCha20Poly1305 };

pub struct EncryptedBytes {
	inner: Vec<u8>
}

pub struct Nonce {
	inner: chacha20poly1305::XNonce
}

pub struct Key {
	inner: chacha20poly1305::Key
}

pub fn encrypt(
	bytes: &[u8],
	key: &Key,
	nonce: &Nonce
) -> Result<EncryptedBytes> {
	let cipher = XChaCha20Poly1305::new(&key.inner);
	let encrypted = cipher.encrypt(&nonce.inner, bytes)?;
	Ok(EncryptedBytes { inner: encrypted })
}

pub fn decrypt(
	bytes: &EncryptedBytes,
	key: &Key,
	nonce: &Nonce
) -> Option<Vec<u8>> {
	let cipher = XChaCha20Poly1305::new(&key.inner);
	cipher.decrypt(&nonce.inner, &*bytes.inner).ok()
}

pub fn generate_nonce() -> Nonce {
	let inner = chacha20poly1305::XNonce::from(util::rand_array());
	Nonce { inner }
}

pub fn generate_key() -> Key {
	let inner = chacha20poly1305::Key::from(util::rand_array());
	Key { inner }
}

pub fn key_from(bytes: &[u8; 32]) -> Key {
	let inner = chacha20poly1305::Key::from(*bytes);
	Key { inner }
}

impl EncryptedBytes {
	pub fn as_bytes(&self) -> &[u8] {
		&self.inner
	}

	pub fn into_bytes(self) -> Vec<u8> {
		self.inner
	}
}
