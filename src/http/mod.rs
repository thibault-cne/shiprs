#![allow(dead_code)]

mod iter;
#[macro_use]
mod macros;
pub mod parser;
pub mod request;
pub mod response;
pub mod uri;

const CR: u8 = b'\r';
const LF: u8 = b'\n';
const CRLF: &[u8; 2] = b"\r\n";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Get => f.write_str("GET"),
            Method::Post => f.write_str("POST"),
            Method::Put => f.write_str("PUT"),
            Method::Delete => f.write_str("DELETE"),
            Method::Head => f.write_str("HEAD"),
        }
    }
}

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
