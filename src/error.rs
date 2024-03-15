use std::error::Error as StdError;

use serde_json::Error as SerdeJsonError;

pub type Result<T> = std::result::Result<T, Error>;

type Cause = Box<dyn StdError + Send + Sync>;

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
    HttpParsing(HttpParsingErrorKind),
    SerdeJson,
}

#[derive(Debug)]
pub(crate) enum HttpParsingErrorKind {
    Response,
    Header,
    Status,
    ChunkSize,
    Chunk,
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

    fn description(&self) -> &str {
        match self.inner.kind {
            ErrorKind::Io => "io error",
            ErrorKind::Unit => "unit error",
            ErrorKind::HttpParsing(ref kind) => kind.description(),
            ErrorKind::SerdeJson => "serde_json error",
        }
    }
}

impl HttpParsingErrorKind {
    fn description(&self) -> &str {
        use HttpParsingErrorKind::*;

        match self {
            Response => "invalid response",
            Header => "invalid header",
            Status => "invalid status",
            ChunkSize => "invalid chunk size",
            Chunk => "invalid chunk",
        }
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
        f.write_str(self.description())
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
    inner_kind HttpParsingErrorKind => HttpParsing;
    with_cause SerdeJsonError => SerdeJson;
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
