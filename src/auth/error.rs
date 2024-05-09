pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
	inner: ErrorInner
}

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
	#[error("{0}")]
	Aead(chacha20poly1305::aead::Error),
	#[error("{0}")]
	Argon2(argon2::Error),
	#[error(transparent)]
	P384PKCS8DER(#[from] p384::pkcs8::der::Error),
	// #[error(transparent)]
	// ParseInt(#[from] std::num::ParseIntError),
	// #[error("error parsing KDF string")]
	// ParseKdf,
	#[error(transparent)]
	TryIntoArray(#[from] std::array::TryFromSliceError),
	// #[error(transparent)]
	// Z85Decode(#[from] crate::z85::DecodeError)
}

impl<T: Into<ErrorInner>> From<T> for Error {
	fn from(inner: T) -> Self {
		Self { inner: inner.into() }
	}
}

impl From<argon2::Error> for ErrorInner {
	fn from(error: argon2::Error) -> Self {
		Self::Argon2(error)
	}
}

impl From<chacha20poly1305::aead::Error> for ErrorInner {
	fn from(error: chacha20poly1305::aead::Error) -> Self {
		Self::Aead(error)
	}
}
