#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
	inner: ErrorInner
}

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
	#[error("{0}")]
	Argon2(argon2::Error),
	#[error(transparent)]
	ParseInt(#[from] std::num::ParseIntError),
	#[error("error parsing KDF string")]
	ParseKdf,
	#[error("unable to convert to fixed length array")]
	TryIntoArray,
	#[error(transparent)]
	Z85Decode(#[from] crate::z85::DecodeError)
}

impl Error {
	#[inline]
	pub(in crate::auth) fn parse_kdf() -> Self {
		let inner = ErrorInner::ParseKdf;
		Self { inner }
	}

	pub(in crate::auth) fn try_into_array() -> Self {
		let inner = ErrorInner::TryIntoArray;
		Self { inner }
	}
}

impl<T> From<T> for Error
where
	ErrorInner: From<T>
{
	fn from(inner: T) -> Self {
		Self { inner: inner.into() }
	}
}

impl From<argon2::Error> for ErrorInner {
	fn from(error: argon2::Error) -> Self {
		Self::Argon2(error)
	}
}
