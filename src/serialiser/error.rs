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

#[inline]
pub fn err_eof<T>() -> Result<T> {
	err(UNEXPECTED_EOF)
}

pub trait OptionExt<T>: Sized {
	fn err<S: Into<String>>(self, s: S) -> Result<T>;

	#[inline]
	fn err_eof(self) -> Result<T> {
		self.err(UNEXPECTED_EOF)
	}
}

impl<T> OptionExt<T> for Option<T> {
	#[inline]
	fn err<S: Into<String>>(self, s: S) -> Result<T> {
		match self {
			Some(v) => { Ok(v) }
			None => { err(s) }
		}
	}
}

pub trait ResultExt<T> {
	fn convert_err(self) -> Result<T>;
}

impl<T, E: ToString> ResultExt<T> for Result<T, E> {
	#[inline]
	fn convert_err(self) -> Result<T> {
		self.map_err(#[inline] |e| error(e.to_string()))
	}
}
