use std::collections::HashMap;
use std::io::{BufReader, Read};

use serde::Deserialize;

use crate::error::{Error, Result};
use crate::http::HttpVersion;

use super::parser::ResponseParser;

// This implementation of HTTP response parsing is mostly taken from
// https://github.com/fristonio/docker.rs
// with minor changes.

/// An HTTP partial response.
/// This is used to parse the response line and headers.
/// The body is not parsed here and is left inside the buffer.
#[derive(Debug)]
pub struct PartialResponse<R> {
    version: HttpVersion,
    status: u16,
    reason: String,
    headers: HashMap<String, String>,
    body_reader: BufReader<R>,
}

/// An HTTP response.
#[derive(Debug, Default)]
pub struct Response<B> {
    version: HttpVersion,
    status: u16,
    reason: String,
    headers: HashMap<String, String>,
    body: Option<B>,
}

impl<B> Response<B> {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> Option<&B> {
        self.body.as_ref()
    }

    pub fn into_body(self) -> Option<B> {
        self.body
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

impl<B, R> TryFrom<BufReader<R>> for Response<B>
where
    R: Read,
    for<'de> B: Deserialize<'de>,
{
    type Error = Error;

    fn try_from(reader: BufReader<R>) -> Result<Self> {
        let mut parser = ResponseParser::new(reader);
        let (version, status, reason, headers) = parser.parse_until_headers()?;
        let body = parser.parse_body()?;

        let body = match (body, status) {
            (Some(_), 204 | 304) => None,
            (Some(body), 200..=399) => Some(serde_json::from_slice::<B>(&body)?),
            (Some(body), 400..=599) => {
                let error = serde_json::from_slice::<shiprs_models::models::ErrorResponse>(&body)?;
                return Err(error.into());
            }
            _ => None,
        };

        Ok(Response {
            version,
            status,
            reason,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_respons_with_chunked_body() -> Result<()> {
        let response: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nTransfer-Encoding: chunked\r\n\r\n5\r\n\"Wiki\r\n7\r\npedia i\r\nA\r\nn chunks.\"\r\n0\r\n\r\n";
        let response = Response::<String>::try_from(BufReader::new(response))?;

        assert_eq!(response.version, HttpVersion::Http1_1);
        assert_eq!(response.status, 200);
        assert_eq!(response.reason, "OK");
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"text/plain".to_string())
        );
        assert_eq!(
            response.headers.get("Transfer-Encoding"),
            Some(&"chunked".to_string())
        );
        assert_eq!(response.body, Some("Wikipedia in chunks.".to_string()));

        Ok(())
    }

    #[test]
    fn test_parse_response_with_length_body() -> Result<()> {
        let response: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 15\r\n\r\n\"Hello, World!\"";
        let response = Response::<String>::try_from(BufReader::new(response))?;

        assert_eq!(response.version, HttpVersion::Http1_1);
        assert_eq!(response.status, 200);
        assert_eq!(response.reason, "OK");
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"text/plain".to_string())
        );
        assert_eq!(
            response.headers.get("Content-Length"),
            Some(&"15".to_string())
        );
        assert_eq!(response.body, Some("Hello, World!".to_string()));

        Ok(())
    }

    #[test]
    fn parse_response_with_empty_body() -> Result<()> {
        let response: &[u8] =
            b"HTTP/1.1 204 No-Content\r\nContent-Type: none\r\nVersion: v1.44\r\n\r\n";
        let response = Response::<()>::try_from(BufReader::new(response))?;

        assert_eq!(response.version, HttpVersion::Http1_1);
        assert_eq!(response.status, 204);
        assert_eq!(response.reason, "No-Content");
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"none".to_string())
        );
        assert_eq!(response.headers.get("Version"), Some(&"v1.44".to_string()));
        assert_eq!(response.body, None);

        Ok(())
    }
}
