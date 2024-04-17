pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Clone, Debug, ::thiserror::Error)]
#[error("{0}")]
pub struct Error(String);

const UNEXPECTED_EOF: &str = "unexpected EOF";

#[inline]
pub fn error<S: Into<String>>(s: S) -> Error {
	Error(s.into())
}

#[inline]
pub fn err<T, S: Into<String>>(s: S) -> Result<T> {
	Err(error(s))
}

pub fn err_eof<T>() -> Result<T> {
	err(UNEXPECTED_EOF)
}

pub trait ResultExt<T>: Sized {
	fn err<S: Into<String>>(self, s: S) -> Result<T>;

	#[inline]
	fn err_eof(self) -> Result<T> {
		self.err(UNEXPECTED_EOF)
	}
}

impl<T> ResultExt<T> for Option<T> {
	#[inline]
	fn err<S: Into<String>>(self, s: S) -> Result<T> {
		match self {
			Some(v) => { Ok(v) }
			None => { err(s) }
		}
	}
}
