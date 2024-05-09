use super::*;

/// Made by hashing the password key with another salt (password verifier salt).
/// Sent to the server to use for initial verification purposes. Server stores
/// it after hashing it again
pub struct PasswordVerifier {
	inner: kdf::Kdf
}

/// Salt used to hash the password key. Sent to server and stored as is
pub struct PasswordVerifierSalt {
	inner: salt::Salt
}

pub(crate) fn new(
	pw_key: &password_key::PasswordKey
) -> Result<(PasswordVerifier, PasswordVerifierSalt)> {
	let salt = generate_salt();
	with_salt(pw_key, &salt).map(|pw_verifier| (pw_verifier, salt))
}

pub(crate) fn with_salt(
	pw_key: &password_key::PasswordKey,
	salt: &PasswordVerifierSalt
) -> Result<PasswordVerifier> {
	let inner = kdf::hash(pw_key.as_bytes(), &salt.inner)?;
	Ok(PasswordVerifier { inner })
}

pub(crate) fn generate_salt() -> PasswordVerifierSalt {
	let inner = salt::generate();
	PasswordVerifierSalt { inner }
}

impl PasswordVerifier {
	pub fn as_bytes(&self) -> &[u8; 32] {
		self.inner.as_hash_bytes()
	}
}
