//! Signin flow functions

use super::*;

pub struct ClientContinuerS1 {
	user_id: user_input::UserID,
	user_password: user_input::UserPassword
}
pub struct ClientContinuerS2 {
	pw_key: password_key::PasswordKey,
	signin_attempt_id: signin_attempt_id::SigninAttemptID
}
pub struct ClientContinuerS3 {
	session_sec_key: session_keypair::SessionSecretKey
}

pub struct SigninRequestS1 {
	pub user_id: user_input::UserID
}
pub struct SigninRequestS2 {
	pub signin_attempt_id: signin_attempt_id::SigninAttemptID,
	pub pw_verifier: password_verifier::PasswordVerifier
}
pub struct SigninRequestS3 {
	pub signin_attempt_id: signin_attempt_id::SigninAttemptID,
	pub challenge_signature: keypair::Signature,
	pub session_pub_key: session_keypair::SessionPublicKey
}

pub struct ServerS2Params {
	pub hashed_pw_verifier: hashed_password_verifier::HashedPasswordVerifier,
	pub hashed_pw_verifier_salt: hashed_password_verifier::HashedPasswordVerifierSalt,
	pub encrypted_user_sec_key: user_keypair::EncryptedUserSecretKey,
	pub encrypted_user_sec_key_nonce: user_keypair::EncryptedUserSecretKeyNonce
}
pub struct ServerS3Params {
	pub signing_challenge: signing_challenge::SigningChallenge,
	pub user_public_key: user_keypair::UserPublicKey
}

pub struct SigninResponseS1 {
	pub signin_attempt_id: signin_attempt_id::SigninAttemptID,
	pub pw_key_salt: password_key::PasswordKeySalt,
	pub pw_verifier_salt: password_verifier::PasswordVerifierSalt
}

pub struct SigninResponseS2 {
	pub encrypted_user_sec_key: user_keypair::EncryptedUserSecretKey,
	pub encrypted_user_sec_key_nonce: user_keypair::EncryptedUserSecretKeyNonce,
	pub signing_challenge: signing_challenge::SigningChallenge
}
pub struct SigninResponseS3 {
	pub session_id: session_id::SessionID
}


pub fn process_signin_client_s1(
	params: user_input::UserDetailsInput
) -> Result<(SigninRequestS1, ClientContinuerS1)> {
	let user_input::UserDetailsInput {
		user_id,
		user_password
	} = params;

	Ok((
		SigninRequestS1 {
			user_id: user_id.clone()
		},
		ClientContinuerS1 {
			user_id,
			user_password
		}
	))
}

pub fn process_signin_client_s2(
	continuer: ClientContinuerS1,
	params: SigninResponseS1
) -> Result<(SigninRequestS2, ClientContinuerS2)> {
	let ClientContinuerS1 {
		user_id,
		user_password
	} = continuer;
	let SigninResponseS1 {
		signin_attempt_id,
		pw_key_salt,
		pw_verifier_salt
	} = params;

	let pw_key = password_key::with_salt(&user_password, &pw_key_salt)?;
	let pw_verifier = password_verifier::with_salt(&pw_key, &pw_verifier_salt)?;

	Ok((
		SigninRequestS2 {
			signin_attempt_id: signin_attempt_id.clone(),
			pw_verifier
		},
		ClientContinuerS2 {
			pw_key,
			signin_attempt_id
		}
	))
}

pub fn process_signin_server_s2(
	req: SigninRequestS2,
	params: ServerS2Params
) -> Result<Option<SigninResponseS2>> {
	let SigninRequestS2 {
		signin_attempt_id,
		pw_verifier
	} = req;
	let ServerS2Params {
		hashed_pw_verifier,
		hashed_pw_verifier_salt,
		encrypted_user_sec_key,
		encrypted_user_sec_key_nonce
	} = params;

	let new_hpv = hashed_password_verifier::with_salt(&pw_verifier, &hashed_pw_verifier_salt)?;
	if hashed_pw_verifier != new_hpv { return Ok(None) }

	let signing_challenge = signing_challenge::generate();

	Ok(Some(SigninResponseS2 {
		encrypted_user_sec_key,
		encrypted_user_sec_key_nonce,
		signing_challenge
	}))
}

pub fn process_signin_client_s3(
	continuer: ClientContinuerS2,
	params: SigninResponseS2
) -> Result<Option<(SigninRequestS3, ClientContinuerS3)>> {
	let ClientContinuerS2 {
		pw_key,
		signin_attempt_id
	} = continuer;
	let SigninResponseS2 {
		encrypted_user_sec_key,
		encrypted_user_sec_key_nonce,
		signing_challenge
	} = params;

	let Some(secret_key) = user_keypair::decrypt_sec_key(
		&encrypted_user_sec_key,
		&pw_key,
		&encrypted_user_sec_key_nonce
	)? else { return Ok(None) };

	let challenge_signature = secret_key.sign(signing_challenge.as_bytes());
	let (session_pub_key, session_sec_key) = session_keypair::generate().into_inner();

	Ok(Some((
		SigninRequestS3 {
			signin_attempt_id,
			challenge_signature,
			session_pub_key
		},
		ClientContinuerS3 {
			session_sec_key
		}
	)))
}

pub fn process_signin_server_s3(
	req: SigninRequestS3,
	params: ServerS3Params
) -> Result<Option<SigninResponseS3>> {
	let SigninRequestS3 {
		signin_attempt_id,
		challenge_signature,
		session_pub_key
	} = req;
	let ServerS3Params {
		signing_challenge,
		user_public_key
	} = params;

	let valid = user_public_key.verify(signing_challenge.as_bytes(), &challenge_signature);
	if !valid { return Ok(None) }

	let session_id = session_id::generate();

	Ok(Some(SigninResponseS3 {
		session_id
	}))
}

pub fn process_signin_client_final(
	continuer: ClientContinuerS3,
	params: SigninResponseS3
) {
	let ClientContinuerS3 { session_sec_key } = continuer;
	let SigninResponseS3 { session_id } = params;
	// TODO: ?????
}
