use super::*;

pub struct SessionKeypair {
	public_key: SessionPublicKey,
	secret_key: SessionSecretKey
}

pub struct SessionPublicKey {
	inner: keypair::PublicKey
}

pub struct SessionSecretKey {
	inner: keypair::SecretKey
}

pub(crate) fn generate() -> SessionKeypair {
	let (public_key, secret_key) = keypair::generate().into_inner();
	let public_key = SessionPublicKey { inner: public_key };
	let secret_key = SessionSecretKey { inner: secret_key };
	SessionKeypair { public_key, secret_key }
}

impl SessionKeypair {
	pub fn into_inner(self) -> (SessionPublicKey, SessionSecretKey) {
		let Self { public_key, secret_key } = self;
		(public_key, secret_key)
	}
}
