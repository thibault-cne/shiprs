use std::error::Error as StdError;

use serde_json::Error as SerdeJsonError;
use shiprs_http::Error as HttpError;

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

type Cause = Box<dyn StdError + Send + Sync>;

/// Internal error type for the Docker API client.
pub struct Error {
    inner: Box<ErrorImpl>,
}

struct ErrorImpl {
    kind: ErrorKind,
    cause: Option<Cause>,
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Io,
    Unit,
    SerdeJson,
    SerdeUrlEncoded,
    ShiprsHttp,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error {
            inner: Box::new(ErrorImpl { kind, cause: None }),
        }
    }

    pub(crate) fn io() -> Error {
        Error::new(ErrorKind::Io)
    }

    pub(crate) fn with<C: Into<Cause>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_tuple("error::Error");
        f.field(&self.inner.kind);
        if let Some(ref cause) = self.inner.cause {
            f.field(cause);
        }
        f.finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;

        match self.inner.kind {
            Io => write!(f, "io error: {}", self.source().unwrap()),
            Unit => write!(f, "unit error"),
            SerdeJson => write!(f, "serde_json error: {}", self.source().unwrap()),
            SerdeUrlEncoded => write!(f, "serde_urlencoded error: {}", self.source().unwrap()),
            ShiprsHttp => write!(f, "shiprs_http error: {}", self.source().unwrap()),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.cause.as_ref().map(|cause| &**cause as _)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::new(ErrorKind::Unit)
    }
}

use macros::error_from;

error_from! {
    with_cause std::io::Error => fn io;
    with_cause SerdeJsonError => SerdeJson;
    with_cause serde_urlencoded::ser::Error => SerdeUrlEncoded;
    with_cause HttpError => ShiprsHttp;
}

mod macros {
    macro_rules! error_from {
        {inner_kind $inner:ident => $kind:ident; $($tt:tt)*} => {
            impl From<$inner> for Error {
                fn from(value: $inner) -> Self {
                    Error::new(ErrorKind::$kind(value))
                }
            }

            error_from!{ $($tt)* }
        };
        {with_cause $error:path => fn $fn:ident; $($tt:tt)*} => {
            impl From<$error> for Error {
                fn from(value: $error) -> Self {
                    Error::$fn().with(value)
                }
            }

            error_from!{ $($tt)* }
        };
        {with_cause $error:path => $kind:ident; $($tt:tt)*} => {
            impl From<$error> for Error {
                fn from(value: $error) -> Self {
                    Error::new(ErrorKind::$kind).with(value)
                }
            }

            error_from!{ $($tt)* }
        };
        {$error:path => $kind:ident; $($tt:tt)*} => {
            impl From<$error> for Error {
                fn from(_: $error) -> Self {
                    Error::new(ErrorKind::$kind)
                }
            }

            error_from!{ $($tt)* }
        };
        {} => {};
    }

    pub(super) use error_from;
}
