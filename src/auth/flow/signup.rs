//! Signup flow functions

use super::*;

pub struct SignupRequest {
	pub user_id: user_input::UserID,
	pub pw_key_salt: password_key::PasswordKeySalt,
	pub pw_verifier: password_verifier::PasswordVerifier,
	pub pw_verifier_salt: password_verifier::PasswordVerifierSalt,
	pub user_public_key: user_keypair::UserPublicKey,
	pub encrypted_user_sec_key: user_keypair::EncryptedUserSecretKey,
	pub encrypted_user_sec_key_nonce: user_keypair::EncryptedUserSecretKeyNonce
}

pub struct StoredUnverifiedUserData {
	pub user_id: user_input::UserID,
	pub pw_key_salt: password_key::PasswordKeySalt,
	pub pw_verifier_salt: password_verifier::PasswordVerifierSalt,
	pub hashed_pw_verifier: hashed_password_verifier::HashedPasswordVerifier,
	pub hashed_pw_verifier_salt: hashed_password_verifier::HashedPasswordVerifierSalt,
	pub user_public_key: user_keypair::UserPublicKey,
	pub encrypted_user_sec_key: user_keypair::EncryptedUserSecretKey,
	pub encrypted_user_sec_key_nonce: user_keypair::EncryptedUserSecretKeyNonce,
	pub verification_token: verification_token::VerificationToken
}

pub fn process_signup_client(params: user_input::UserDetailsInput) -> Result<SignupRequest> {
	let user_input::UserDetailsInput { user_id, user_password } = params;

	let (user_public_key, user_secret_key) = user_keypair::generate().into_inner();
	let (pw_key, pw_key_salt) = password_key::new(&user_password)?;
	let (pw_verifier, pw_verifier_salt) = password_verifier::new(&pw_key)?;

	let (
		encrypted_user_sec_key,
		encrypted_user_sec_key_nonce
	) = user_secret_key.encrypt(&pw_key)?;

	Ok(SignupRequest {
		user_id,
		pw_key_salt,
		pw_verifier,
		pw_verifier_salt,
		user_public_key,
		encrypted_user_sec_key,
		encrypted_user_sec_key_nonce
	})
}

/// Call with data received from the client
pub fn process_signup_server(params: SignupRequest) -> Result<StoredUnverifiedUserData> {
	let SignupRequest {
		user_id,
		pw_key_salt,
		pw_verifier,
		pw_verifier_salt,
		user_public_key,
		encrypted_user_sec_key,
		encrypted_user_sec_key_nonce
	} = params;

	let (hashed_pw_verifier, hashed_pw_verifier_salt) = hashed_password_verifier::new(&pw_verifier)?;
	let verification_token = verification_token::generate();

	Ok(StoredUnverifiedUserData {
		user_id,
		pw_key_salt,
		pw_verifier_salt,
		hashed_pw_verifier,
		hashed_pw_verifier_salt,
		user_public_key,
		encrypted_user_sec_key,
		encrypted_user_sec_key_nonce,
		verification_token
	})
}
