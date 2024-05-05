//! Signup flow functions

use super::*;

/// The part of user signup form needed for verification purposes
pub struct SignupClientIn {
	pub user_id: user_details::UserID,
	pub user_password: user_details::UserPassword
}

pub fn process_signup_client(params: SignupClientIn) -> Result<SignupServerReq> {
	let SignupClientIn { user_id, user_password } = params;

	let user_keypair = UserKeypair::generate();

	let password_salt = UserPasswordSalt::generate();
	let password_key = PasswordKey::new(&user_password, &password_salt)?;

	let password_verifier_salt = PasswordVerifierSalt::generate();
	let password_verifier = PasswordVerifier::new(&
		password_key,
		&password_verifier_salt
	)?;

	let user_secret_key_nonce = UserSecretKeyNonce::generate();
	let encrypted_user_secret_key = EncryptedUserSecretKey::new(
		&user_keypair,
		&password_key,
		&user_secret_key_nonce
	)?;

	Ok(SignupServerReq {
		user_id,
		password_salt,
		password_verifier_salt,
		password_verifier,
		user_secret_key_nonce,
		encrypted_user_secret_key,
		user_public_key: user_keypair.public_key
	})
}

/// Data sent by the client to the server.
pub struct SignupServerReq {
	pub user_id: UserID,
	pub password_salt: UserPasswordSalt,

	pub password_verifier_salt: PasswordVerifierSalt,
	pub password_verifier: PasswordVerifier,
	pub user_secret_key_nonce: UserSecretKeyNonce,
	pub encrypted_user_secret_key: EncryptedUserSecretKey,
	pub user_public_key: UserPublicKey
}

/// Call with data received from the client
pub fn process_signup_server(params: SignupServerReq) -> EnsureIDUnique {
	EnsureIDUnique { params }
}

/// Call [`confirmed_user_id_unique`][Self::confirmed_user_id_unique] after the
/// server has confirmed the user ID has not been taken already (by unverified or
/// verified users), and reserved it (so no one else can snipe it while server
/// isn't looking)
pub struct EnsureIDUnique {
	params: SignupServerReq
}

impl EnsureIDUnique {
	/// Get user ID to check for uniqueness
	pub fn user_id(&self) -> &str {
		&self.params.user_id.inner
	}

	/// Call this once the server has confirmed that user ID is unique and
	/// reserved it
	pub fn confirmed_user_id_unique(self) -> Result<SendVerification> {
		let SignupServerReq {
			user_id,
			password_salt,
			password_verifier_salt,
			password_verifier,
			user_secret_key_nonce,
			encrypted_user_secret_key,
			user_public_key
		} = self.params;

		let hashed_password_verifier_salt = HashedPasswordVerifierSalt::generate();
		let hashed_password_verifier = HashedPasswordVerifier::new(
			&password_verifier,
			&hashed_password_verifier_salt
		)?;

		let verification_token = VerificationToken::generate();
		let last_token_generate_time = LastVerificationTokenGenerationTime::now();

		let stored_data = ServerStoredUnverifiedUserData {
			user_id,
			password_salt,
			hashed_password_verifier_salt,
			hashed_password_verifier,
			password_verifier_salt,
			user_secret_key_nonce,
			encrypted_user_secret_key,
			user_public_key,
			verification_token,
			last_token_generate_time
		};
		Ok(SendVerification { stored_data })
	}
}

pub struct SendVerification {
	stored_data: ServerStoredUnverifiedUserData
}

impl SendVerification {
	pub fn user_id(&self) -> &str {
		&self.stored_data.user_id.inner
	}

	pub fn verification_token(&self) -> &[u8; 32] {
		&self.stored_data.verification_token.inner
	}

	pub fn verification_sent(self) -> ServerStoredUnverifiedUserData {
		self.stored_data
	}
}

/// Data the server stores for unverified users
pub struct ServerStoredUnverifiedUserData {
	pub user_id: UserID,
	pub password_salt: UserPasswordSalt,

	pub hashed_password_verifier_salt: HashedPasswordVerifierSalt,
	pub hashed_password_verifier: HashedPasswordVerifier,
	pub password_verifier_salt: PasswordVerifierSalt,
	pub user_secret_key_nonce: UserSecretKeyNonce,
	pub encrypted_user_secret_key: EncryptedUserSecretKey,
	pub user_public_key: UserPublicKey,
	pub verification_token: VerificationToken,
	pub last_token_generate_time: LastVerificationTokenGenerationTime
}
