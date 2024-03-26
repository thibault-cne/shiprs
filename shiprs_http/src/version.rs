#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HttpVersion {
    Http1_0,
    #[default]
    Http1_1,
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http1_0 => f.write_str("HTTP/1.0"),
            HttpVersion::Http1_1 => f.write_str("HTTP/1.1"),
        }
    }
}

impl<S> From<S> for HttpVersion
where
    S: AsRef<[u8]>,
{
    fn from(value: S) -> Self {
        match value.as_ref() {
            b"HTTP/1.0" => HttpVersion::Http1_0,
            b"HTTP/1.1" => HttpVersion::Http1_1,
            _ => panic!("unsupported HTTP version"),
        }
    }
}
