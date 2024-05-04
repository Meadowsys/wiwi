use p384::{ PublicKey, SecretKey };
use rand::thread_rng;
use rand::rngs::adapter::ReseedingRng;

pub struct Keypair {
	public_key: p384::PublicKey,
	secret_key: p384::SecretKey,
	encrypted_secret_key: Option<()>
}

impl Keypair {
	pub(in crate::auth) fn generate() -> Self {
		let secret_key = p384::SecretKey::random(&mut thread_rng());
		let public_key = secret_key.public_key();
		let encrypted_secret_key = None;

		Self { public_key, secret_key, encrypted_secret_key }
	}
}
