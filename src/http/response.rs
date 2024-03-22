use std::collections::HashMap;

use serde::Deserialize;

use crate::error::{Error, HttpParsingErrorKind::*, Result};
use crate::http::{iter::Bytes, HttpVersion, CRLF};

// This implementation of HTTP response parsing is mostly taken from
// https://github.com/fristonio/docker.rs
// with minor changes.

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

impl<'buf, B> Response<B>
where
    for<'de> B: Deserialize<'de>,
{
    fn parse(buf: &'buf [u8]) -> Result<Status<Response<B>>> {
        let mut bytes = Bytes::new(buf);

        let version = complete!(parse_version(&mut bytes));
        space!(bytes or Version.into());
        let status = complete!(parse_status(&mut bytes));
        let reason = match next!(bytes) {
            b' ' => {
                bytes.commit();
                complete!(parse_reason(&mut bytes))
            }
            b'\r' => {
                expect!(bytes.next() == b'\n' => Err(Status.into()));
                ""
            }
            b'\n' => "",
            _ => return Err(Status.into()),
        }
        .to_string();
        bytes.commit();

        let mut headers = HashMap::new();
        complete!(parse_headers(&mut bytes, &mut headers));
        bytes.commit();

        let body_kind = if status == 204 || status == 304 {
            BodyKind::Empty
        } else if headers
            .get("Transfer-Encoding")
            .map_or(false, |enc| enc == "chunked")
        {
            BodyKind::Chunked
        } else {
            BodyKind::Unsupported
        };

        match body_kind {
            BodyKind::Empty => Ok(Status::Complete(Response {
                version,
                status,
                reason,
                headers,
                body: None,
            })),
            BodyKind::Chunked => {
                let body = complete!(parse_chunked_body(&mut bytes));
                let body = Some(serde_json::from_slice::<B>(&body).map_err(Error::from)?);
                Ok(Status::Complete(Response {
                    version,
                    status,
                    reason,
                    headers,
                    body,
                }))
            }
            BodyKind::Unsupported => Err(UnsupportedBodyEncoding.into()),
        }
    }
}

#[derive(Debug)]
enum Status<T> {
    Complete(T),
    Partial,
}

fn parse_version(bytes: &mut Bytes) -> Result<Status<HttpVersion>> {
    if let Some(eight) = bytes.peek_n::<[u8; 8]>() {
        let h10 = u64::from_ne_bytes(*b"HTTP/1.O");
        let h11 = u64::from_ne_bytes(*b"HTTP/1.1");

        // SAFETY: The bytes are guaranteed to be 8 bytes long.
        unsafe {
            bytes.advance(8);
        }

        let version = u64::from_ne_bytes(eight);

        return match version {
            v if v == h10 => Ok(Status::Complete(HttpVersion::Http1_0)),
            v if v == h11 => Ok(Status::Complete(HttpVersion::Http1_1)),
            _ => Err(Version.into()),
        };
    }

    expect!(bytes.next() == b'H' => Err(Version.into()));
    expect!(bytes.next() == b'T' => Err(Version.into()));
    expect!(bytes.next() == b'T' => Err(Version.into()));
    expect!(bytes.next() == b'P' => Err(Version.into()));
    expect!(bytes.next() == b'/' => Err(Version.into()));
    expect!(bytes.next() == b'1' => Err(Version.into()));
    expect!(bytes.next() == b'.' => Err(Version.into()));
    Ok(Status::Partial)
}

#[inline]
fn parse_status(bytes: &mut Bytes<'_>) -> Result<Status<u16>> {
    let hundreds = expect!(bytes.next() == b'0'..=b'9' => Err(Status.into()));
    let tens = expect!(bytes.next() == b'0'..=b'9' => Err(Status.into()));
    let ones = expect!(bytes.next() == b'0'..=b'9' => Err(Status.into()));

    Ok(Status::Complete(
        (hundreds - b'0') as u16 * 100 + (tens - b'0') as u16 * 10 + (ones - b'0') as u16,
    ))
}

#[inline]
fn parse_reason<'buf>(bytes: &mut Bytes<'buf>) -> Result<Status<&'buf str>> {
    let mut seen_obs_text = false;
    loop {
        let b = next!(bytes);
        if b == b'\r' {
            expect!(bytes.next() == b'\n' => Err(Reason.into()));
            return Ok(Status::Complete(
                // SAFETY: (1) calling bytes.slice_skip(2) is safe, because at least two next! calls
                // advance the bytes iterator.
                // (2) calling from_utf8_unchecked is safe, because the bytes returned by slice_skip
                // were validated to be allowed US-ASCII chars by the other arms of the if/else or
                // otherwise `seen_obs_text` is true and an empty string is returned instead.
                unsafe {
                    let bytes = bytes.slice_skip(2);
                    if !seen_obs_text {
                        // all bytes up till `i` must have been HTAB / SP / VCHAR
                        std::str::from_utf8_unchecked(bytes)
                    } else {
                        // obs-text characters were found, so return the fallback empty string
                        ""
                    }
                },
            ));
        } else if b == b'\n' {
            return Ok(Status::Complete(
                // SAFETY: (1) calling bytes.slice_skip(1) is safe, because at least one next! call
                // advance the bytes iterator.
                // (2) see (2) of safety comment above.
                unsafe {
                    let bytes = bytes.slice_skip(1);
                    if !seen_obs_text {
                        // all bytes up till `i` must have been HTAB / SP / VCHAR
                        std::str::from_utf8_unchecked(bytes)
                    } else {
                        // obs-text characters were found, so return the fallback empty string
                        ""
                    }
                },
            ));
        } else if !(b == 0x09 || b == b' ' || (0x21..=0x7E).contains(&b) || b >= 0x80) {
            return Err(Reason.into());
        } else if b >= 0x80 {
            seen_obs_text = true;
        }
    }
}

#[inline]
fn parse_headers(bytes: &mut Bytes, map: &mut HashMap<String, String>) -> Result<Status<usize>> {
    let start = bytes.as_ref().as_ptr() as usize;

    loop {
        match next!(bytes) {
            b'\r' => {
                expect!(bytes.next() == b'\n' => Err(Header.into()));
                return Ok(Status::Complete(bytes.as_ref().as_ptr() as usize - start));
            }
            b'\n' => return Ok(Status::Complete(bytes.as_ref().as_ptr() as usize - start)),
            b':' => {
                // SAFETY: at least one next! call has advanced the bytes iterator.
                // It's just to remove the colon.
                let name = unsafe { bytes.slice_skip(1) };

                // Remove leading whitespace.
                while bytes.peek() == Some(b' ') {
                    // SAFETY: at least one we've peeked the next byte.
                    unsafe { bytes.bump() };
                }
                bytes.commit();
                loop {
                    match next!(bytes) {
                        b'\r' => {
                            expect!(bytes.next() == b'\n' => Err(Header.into()));
                            break;
                        }
                        b'\n' => break,
                        _ => continue,
                    }
                }

                // SAFETY: at least two next! calls have advanced the bytes iterator.
                let value = unsafe { bytes.slice_skip(2) };
                map.insert(
                    // SAFETY: the bytes returned by slice are valid US-ASCII chars.
                    unsafe { std::str::from_utf8_unchecked(name) }.to_string(),
                    // SAFETY: the bytes returned by slice are valid US-ASCII chars.
                    unsafe { std::str::from_utf8_unchecked(value) }.to_string(),
                );
            }
            _ => continue,
        }
    }
}

#[inline]
fn parse_chunked_body(bytes: &mut Bytes) -> Result<Status<Vec<u8>>> {
    let mut body = Vec::new();

    loop {
        match next!(bytes) {
            b'\r' => {
                expect!(bytes.next() == b'\n' => Err(ChunkSize.into()));
                // SAFETY: (1) calling bytes.slice_skip(2) is safe, because at least two next! calls have been made
                let chunk_size = unsafe { bytes.slice_skip(2) };
                let chunk_size_s = unsafe { std::str::from_utf8_unchecked(chunk_size) };
                let chunk_size = usize::from_str_radix(chunk_size_s, 16)
                    .map_err(|_| Into::<Error>::into(ChunkSize))?;

                if chunk_size == 0 && bytes.peek_n::<[u8; 2]>() == Some(*CRLF) {
                    return Ok(Status::Complete(body));
                }

                // SAFETY: it's safe to advance the bytes iterator by `chunk_size` bytes, because
                // the chunk size was validated to be a valid hexadecimal number and the bytes
                // iterator is guaranteed to have at least `chunk_size` bytes left.
                if bytes.len() < chunk_size {
                    return Err(Chunk.into());
                }
                unsafe { bytes.advance(chunk_size) };
                let chunk = bytes.slice();
                body.extend_from_slice(chunk);
                newline!(bytes);
            }
            _ => continue,
        }
    }
}

enum BodyKind {
    Chunked,
    Empty,
    Unsupported,
}

impl<'buf, R> TryFrom<&'buf [u8]> for Response<R>
where
    for<'de> R: Deserialize<'de>,
{
    type Error = Error;

    fn try_from(value: &'buf [u8]) -> Result<Self> {
        match Response::parse(value)? {
            Status::Complete(response) => Ok(response),
            Status::Partial => Err(Response.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() -> Result<()> {
        let version = b"HTTP/1.1 200 OK\r\n";
        let mut version = Bytes::new(version);

        let result = parse_version(&mut version)?;

        match result {
            Status::Complete(version) => {
                assert_eq!(version, HttpVersion::Http1_1);
            }
            Status::Partial => {
                panic!("Expected complete result");
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_status() -> Result<()> {
        let status = b"200 OK\r\n";
        let mut status = Bytes::new(status);

        let result = parse_status(&mut status)?;

        match result {
            Status::Complete(status) => {
                assert_eq!(status, 200);
            }
            Status::Partial => {
                panic!("Expected complete result");
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_reason() -> Result<()> {
        let reason = b"OK\r\n";
        let mut reason = Bytes::new(reason);

        let result = parse_reason(&mut reason)?;

        match result {
            Status::Complete(reason) => {
                assert_eq!(reason, "OK");
            }
            Status::Partial => {
                panic!("Expected complete result");
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_headers() -> Result<()> {
        let headers = b"Content-Type: text/plain\r\nContent-Length: 12\r\n\r\n";
        let mut headers = Bytes::new(headers);
        let mut map = HashMap::new();

        let result = parse_headers(&mut headers, &mut map)?;

        match result {
            Status::Complete(_) => {
                assert_eq!(map.get("Content-Type"), Some(&"text/plain".to_string()));
                assert_eq!(map.get("Content-Length"), Some(&"12".to_string()));
            }
            Status::Partial => {
                panic!("Expected complete result");
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_chunked_body() -> Result<()> {
        let bytes = b"4\r\nWiki\r\n7\r\npedia i\r\nB\r\nn \r\nchunks.\r\n0\r\n\r\n";
        let mut bytes = Bytes::new(bytes);
        let result = parse_chunked_body(&mut bytes)?;

        match result {
            Status::Complete(body) => {
                assert_eq!(body, b"Wikipedia in \r\nchunks.");
            }
            Status::Partial => {
                panic!("Expected complete result");
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_respons_with_body() -> Result<()> {
        let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nTransfer-Encoding: chunked\r\n\r\n5\r\n\"Wiki\r\n7\r\npedia i\r\nA\r\nn chunks.\"\r\n0\r\n\r\n";

        let result = Response::<String>::parse(response)?;

        match result {
            Status::Complete(response) => {
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
            }
            Status::Partial => {
                panic!("Expected complete result");
            }
        }

        Ok(())
    }

    #[test]
    fn parse_response_with_empty_body() -> Result<()> {
        let response = b"HTTP/1.1 204 No-Content\r\nContent-Type: none\r\nVersion: v1.44\r\n\r\n";

        let result = Response::<()>::parse(response)?;

        match result {
            Status::Complete(response) => {
                assert_eq!(response.version, HttpVersion::Http1_1);
                assert_eq!(response.status, 204);
                assert_eq!(response.reason, "No-Content");
                assert_eq!(
                    response.headers.get("Content-Type"),
                    Some(&"none".to_string())
                );
                assert_eq!(response.headers.get("Version"), Some(&"v1.44".to_string()));
                assert_eq!(response.body, None);
            }
            Status::Partial => {
                panic!("Expected complete result");
            }
        }

        Ok(())
    }

    #[test]
    fn test_deserialize_body() -> Result<()> {
        let bytes = b"5\r\n\"Wiki\r\n7\r\npedia i\r\nA\r\nn chunks.\"\r\n0\r\n\r\n";
        let mut bytes = Bytes::new(bytes);

        let body = match parse_chunked_body(&mut bytes)? {
            Status::Complete(body) => body,
            Status::Partial => panic!("Expected complete result"),
        };

        let expected = b"Wikipedia in chunks.";
        let expected_s = unsafe { std::str::from_utf8_unchecked(expected) };

        let parsed_body = serde_json::from_slice::<&str>(&body)?;
        assert_eq!(parsed_body, expected_s);

        Ok(())
    }
}
