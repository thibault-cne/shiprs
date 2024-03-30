use std::error::Error as StdError;

use serde_json::Error as SerdeJsonError;
use shiprs_http::Error as HttpError;

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Internal error type for the Docker API client.
pub struct Error {
    inner: ErrorImpl,
}

struct ErrorImpl {
    kind: ErrorKind,
    error: Box<dyn StdError + Send + Sync>,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    Io,
    SerdeJson,
    SerdeUrlEncoded,
    ShiprsHttp,
    DockerApiResponse,
}

impl Error {
    pub(crate) fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        Error {
            inner: ErrorImpl {
                kind,
                error: error.into(),
            },
        }
    }

    pub(crate) fn docker_api_response(
        status: u16,
        inner: shiprs_models::models::ErrorResponse,
    ) -> Error {
        Error::new(
            ErrorKind::DockerApiResponse,
            DockerApiResponse { status, inner },
        )
    }

    pub fn kind(&self) -> ErrorKind {
        self.inner.kind
    }

    pub fn into_inner(self) -> Box<dyn StdError + Send + Sync> {
        self.inner.error
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_tuple("error::Error");
        f.field(&self.inner.kind);
        f.field(&self.inner.error);
        f.finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.error.fmt(f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.error.as_ref())
    }
}

#[derive(Debug)]
pub struct DockerApiResponse {
    pub inner: shiprs_models::models::ErrorResponse,
    pub status: u16,
}

impl std::fmt::Display for DockerApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.message)
    }
}

impl StdError for DockerApiResponse {}

use macros::error_from;

error_from! {
    std::io::Error => Io;
    SerdeJsonError => SerdeJson;
    serde_urlencoded::ser::Error => SerdeUrlEncoded;
    HttpError => ShiprsHttp;
}

mod macros {
    macro_rules! error_from {
        {} => {};
        {$error:path => fn $fn:ident; $($tt:tt)*} => {
            impl From<$error> for Error {
                fn from(value: $error) -> Self {
                    Error::$fn(value)
                }
            }

            error_from!{ $($tt)* }
        };
        {$error:path => $kind:ident; $($tt:tt)*} => {
            impl From<$error> for Error {
                fn from(value: $error) -> Self {
                    Error::new(ErrorKind::$kind, value)
                }
            }

            error_from!{ $($tt)* }
        };
    }

    pub(super) use error_from;
}
