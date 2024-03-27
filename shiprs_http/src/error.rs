use std::error::Error as StdError;

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

type Cause = Box<dyn StdError + Send + Sync>;

/// Internal error type for the Docker API client.
pub struct Error {
    inner: Box<ErrorImpl>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error {
            inner: Box::new(ErrorImpl { kind, cause: None }),
        }
    }

    pub(crate) fn with<C: Into<Cause>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }
}

struct ErrorImpl {
    kind: ErrorKind,
    cause: Option<Cause>,
}

#[derive(Debug)]
pub enum ErrorKind {
    SerdeUrlEncoded,
    SerdeJson,
    Io,
    HttpParsing(HttpParsingKind),
}

#[derive(Debug)]
pub enum HttpParsingKind {
    Version,
    Status,
    Reason,
    Header,
    ChunkSize,
    Chunk,
    ContentLength,
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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::new(ErrorKind::Io).with(err)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        Error::new(ErrorKind::SerdeUrlEncoded).with(err)
    }
}

impl From<HttpParsingKind> for Error {
    fn from(kind: HttpParsingKind) -> Self {
        Error::new(ErrorKind::HttpParsing(kind))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeJson).with(err)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.cause.as_ref().map(|c| &**c as _)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;

        match self.inner.kind {
            Io => write!(f, "io error: {}", self.source().unwrap()),
            SerdeUrlEncoded => write!(f, "serde_urlencoded error: {}", self.source().unwrap()),
            SerdeJson => write!(f, "serde_json error: {}", self.source().unwrap()),
            HttpParsing(ref kind) => write!(f, "http parsing error: {:?}", kind),
        }
    }
}
