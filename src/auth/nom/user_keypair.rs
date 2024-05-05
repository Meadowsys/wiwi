use super::*;

/// Keypair for a user, used to help authenticate them. Generated on user's device,
/// so the server never has the secret key. Public key is sent to the server as
/// is for storage/verification later
pub struct UserKeypair {
	pub(in crate::auth) public_key: UserPublicKey,
	pub(in crate::auth) secret_key: UserSecretKey
}

/// User's public key, stored plain
pub struct UserPublicKey {
	pub(in crate::auth) inner: keypair::PublicKey
}

/// User's secret key, never sent over the network without encrypting
pub struct UserSecretKey {
	pub(in crate::auth) inner: keypair::SecretKey
}

impl UserKeypair {
	pub(in crate::auth) fn generate() -> Self {
		let keypair::Keypair { public_key, secret_key } = keypair::Keypair::generate();
		let public_key = UserPublicKey { inner: public_key };
		let secret_key = UserSecretKey { inner: secret_key };
		Self { public_key, secret_key }
	}
}

/// Nonce used in encrypting the secret key. Sent to server for storage as is.
pub struct UserSecretKeyNonce {
	pub(in crate::auth) inner: chacha::Nonce
}

impl UserSecretKeyNonce {
	pub(in crate::auth) fn generate() -> Self {
		let inner = chacha::Nonce::generate();
		Self { inner }
	}
}

/// Secret key of the user keypair, in encrypted form. Hashed with the password
/// key. Sent to server and stored as is, and retrieved from server to use for
/// authentication later
pub struct EncryptedUserSecretKey {
	pub(in crate::auth) inner: chacha::ChaCha
}

impl EncryptedUserSecretKey {
	pub(in crate::auth) fn new(
		keypair: &UserKeypair,
		password_key: &password_key::PasswordKey,
		sec_key_nonce: &UserSecretKeyNonce
	) -> Result<Self> {
		let inner = chacha::ChaCha::encrypt(
			&keypair.secret_key.inner.to_bytes()?,
			&chacha::Key::from_bytes(password_key.inner.as_bytes()),
			&sec_key_nonce.inner
		)?;
		Ok(Self { inner })
	}
}
