use super::*;

/// Made by hashing the password key with another salt (password verifier salt).
/// Sent to the server to use for initial verification purposes. Server stores
/// it after hashing it again
pub struct PasswordVerifier {
	pub(in crate::auth) inner: kdf::Kdf
}

impl PasswordVerifier {
	pub(in crate::auth) fn new(
		pw_key: &password_key::PasswordKey,
		salt: &PasswordVerifierSalt
	) -> Result<Self> {
		let inner = kdf::Kdf::hash_and_salt(pw_key.inner.as_bytes(), &salt.inner)?;
		Ok(Self { inner })
	}
}

/// Salt used to hash the password key. Sent to server and stored as is
pub struct PasswordVerifierSalt {
	pub(in crate::auth) inner: salt::Salt
}

impl PasswordVerifierSalt {
	pub(in crate::auth) fn generate() -> Self {
		let inner = salt::Salt::generate();
		Self { inner }
	}
}
