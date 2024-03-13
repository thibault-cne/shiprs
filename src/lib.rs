use std::os::unix::net::UnixStream;

mod container;
#[cfg(feature = "chrono")]
mod datetime;
mod docker;
mod error;
mod http;
mod image;
mod models;
mod network;
mod transport;

const API_VERSION: &str = "v1.44";

pub use docker::Docker;
pub use error::{Error, Result};
pub use http::{
    request::{Request, RequestBuilder},
    response::Response,
    Method,
};

/// A docker socket.
pub enum Socket {
    Unix(String),
}

impl Socket {
    /// Create a new unix socket from a path.
    pub fn unix(path: &str) -> Self {
        Socket::Unix(path.to_string())
    }
}

impl TryInto<UnixStream> for &Socket {
    type Error = std::io::Error;

    fn try_into(self) -> std::result::Result<UnixStream, Self::Error> {
        match self {
            Socket::Unix(path) => UnixStream::connect(path),
        }
    }
}

pub trait OptionTrait {
    fn as_string(&self) -> String;
}
