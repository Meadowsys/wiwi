use super::*;

/// Made by hashing a user's password with the password salt. Used for
/// encrypting/decrypting the secret key. Not sent to the server; only sent after
/// hashing again (as password verifier).
pub struct PasswordKey {
	inner: kdf::Kdf
}

pub struct PasswordKeySalt {
	inner: salt::Salt
}

pub(crate) fn new(
	pw: &user_input::UserPassword
) -> Result<(PasswordKey, PasswordKeySalt)> {
	let salt = generate_salt();
	with_salt(pw, &salt).map(|pw_key| (pw_key, salt))
}

pub(crate) fn with_salt(
	pw: &user_input::UserPassword,
	salt: &PasswordKeySalt
) -> Result<PasswordKey> {
	let inner = kdf::hash(pw.as_bytes(), &salt.inner)?;
	Ok(PasswordKey { inner })
}

pub(crate) fn generate_salt() -> PasswordKeySalt {
	let inner = salt::generate();
	PasswordKeySalt { inner }
}

impl PasswordKey {
	pub(crate) fn as_bytes(&self) -> &[u8; 32] {
		self.inner.as_hash_bytes()
	}
}
