use super::*;

/// Keypair for a user, used to help authenticate them. Generated on user's device,
/// so the server never has the secret key. Public key is sent to the server as
/// is for storage/verification later
pub struct UserKeypair {
	public_key: UserPublicKey,
	secret_key: UserSecretKey
}

/// User's public key, stored plain
pub struct UserPublicKey {
	inner: keypair::PublicKey
}

/// User's secret key, never sent over the network without encrypting
pub struct UserSecretKey {
	inner: keypair::SecretKey
}

// User secret key, encrypted
pub struct EncryptedUserSecretKey {
	inner: chacha::EncryptedBytes
}

pub struct EncryptedUserSecretKeyNonce {
	inner: chacha::Nonce
}

pub(crate) fn generate() -> UserKeypair {
	let (public_key, secret_key) = keypair::generate().into_inner();
	let public_key = UserPublicKey { inner: public_key };
	let secret_key = UserSecretKey { inner: secret_key };
	UserKeypair { public_key, secret_key }
}

pub(crate) fn decrypt_sec_key(
	encrypted: &EncryptedUserSecretKey,
	pw_key: &password_key::PasswordKey,
	nonce: &EncryptedUserSecretKeyNonce
) -> Result<Option<UserSecretKey>> {
	let Some(decrypted) = chacha::decrypt(
		&encrypted.inner,
		&chacha::key_from(pw_key.as_bytes()),
		&nonce.inner
	) else { return Ok(None) };

	let key = keypair::sec_key_from_bytes(&decrypted)?;
	Ok(Some(UserSecretKey { inner: key }))
}

impl UserPublicKey {
	pub(crate) fn verify(&self, bytes: &[u8], sig: &keypair::Signature) -> bool {
		self.inner.verify(bytes, sig)
	}
}

impl UserSecretKey {
	pub(crate) fn encrypt(
		&self,
		pw_key: &password_key::PasswordKey
	) -> Result<(EncryptedUserSecretKey, EncryptedUserSecretKeyNonce)> {
		let nonce = chacha::generate_nonce();
		let encrypted_bytes = chacha::encrypt(
			&self.inner.to_bytes()?,
			&chacha::key_from(pw_key.as_bytes()),
			&nonce
		)?;
		Ok((
			EncryptedUserSecretKey { inner: encrypted_bytes },
			EncryptedUserSecretKeyNonce { inner: nonce }
		))
	}

	pub(crate) fn sign(&self, bytes: &[u8]) -> keypair::Signature {
		self.inner.sign(bytes)
	}
}

impl UserKeypair {
	pub fn public_key(&self) -> &UserPublicKey {
		&self.public_key
	}

	pub fn secret_key(&self) -> &UserSecretKey {
		&self.secret_key
	}

	pub fn into_inner(self) -> (UserPublicKey, UserSecretKey) {
		let Self { public_key, secret_key } = self;
		(public_key, secret_key)
	}
}

// impl UserSecretKeyNonce {
// 	pub(crate) fn generate() -> Self {
// 		let inner = chacha::Nonce::generate();
// 		Self { inner }
// 	}
// }

// impl EncryptedUserSecretKey {
// 	pub(crate) fn new(
// 		keypair: &UserKeypair,
// 		password_key: &password_key::PasswordKey,
// 		sec_key_nonce: &UserSecretKeyNonce
// 	) -> Result<Self> {
// 		let inner = chacha::ChaCha::encrypt(
// 			&keypair.secret_key.inner.to_bytes()?,
// 			&chacha::Key::from_bytes(password_key.inner.as_bytes()),
// 			&sec_key_nonce.inner
// 		)?;
// 		Ok(Self { inner })
// 	}
// }
