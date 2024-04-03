//! Websocket implementation

super::runtime_selection_compile_check!("ws");

// TODO: if/when adding more runtimes, swap tokio AsyncRead for futures AsyncRead
// and use tokio-util package to interface?

#[cfg(feature = "tokio")]
pub use tokio::*;

#[cfg(feature = "tokio")]
mod tokio {
	use ::tokio::net::TcpSocket;
	use tokio::io::AsyncRead;

	pub struct ClientBuilder {}
	pub struct Client {}

	pub struct ServerBuilder {}
	pub struct Server {}

	// pub struct Connection {}

	pub struct ConnectOpts {

	}

	impl Client {
		pub fn builder() -> ClientBuilder {
			todo!()
		}

		pub fn connect(url: &str) {}
	}

	impl Server {
		pub fn builder() -> ServerBuilder {
			todo!()
		}
	}


	use url::Url;
}
