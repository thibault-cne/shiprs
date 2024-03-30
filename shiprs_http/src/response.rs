use std::collections::HashMap;
use std::io::{BufReader, Error as IoError, ErrorKind, Read, Result as IoResult};

use crate::bytes::Bytes;
use crate::error::{Error, HttpParsingKind::*, Result};
use crate::version::HttpVersion;
use crate::{CRLF, HEADERS_END};

/// An HTTP response.
#[derive(Debug, Default, Clone)]
pub struct Response {
    #[allow(dead_code)]
    version: HttpVersion,
    status: u16,
    #[allow(dead_code)]
    reason: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

pub struct Parser<R> {
    inner: BufReader<R>,
    kind: BodyKind,
}

impl<R> Parser<R>
where
    R: Read,
{
    pub fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner),
            kind: BodyKind::Empty,
        }
    }

    fn parse_until_headers(
        &mut self,
    ) -> Result<(HttpVersion, u16, String, HashMap<String, String>)> {
        let buf = self.read_until_headers()?;

        let mut bytes = Bytes::new(&buf);

        let version = parse_version(&mut bytes)?;
        space!(bytes or Version.into());
        let status = parse_status(&mut bytes)?;
        let reason = match next!(bytes => Err(Status.into())) {
            b' ' => {
                bytes.commit();
                parse_reason(&mut bytes)?
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
        parse_headers(&mut bytes, &mut headers)?;
        self.kind = BodyKind::try_from_headers(&headers)?;

        Ok((version, status, reason, headers))
    }

    fn parse_body(&mut self) -> Result<Option<Vec<u8>>> {
        match self.kind {
            BodyKind::Chunked => parse_chunked_body(self).map(Some),
            BodyKind::Length(_) => parse_length_body(self).map(Some),
            BodyKind::Empty => Ok(None),
        }
    }

    fn read_until_headers(&mut self) -> IoResult<Vec<u8>> {
        let mut buf = Vec::new();

        loop {
            let byte = self.inner.by_ref().bytes().next();

            match byte {
                Some(b) => buf.push(b?),
                None => return Err(IoError::new(ErrorKind::ConnectionAborted, "Unexpected EOF")),
            };

            if buf.ends_with(HEADERS_END) {
                break;
            }
        }

        Ok(buf)
    }

    fn read_next_line(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        loop {
            let byte = self.inner.by_ref().bytes().next();

            match byte {
                Some(b) => buf.push(b?),
                None => {
                    return Err(IoError::new(ErrorKind::ConnectionAborted, "Unexpected EOF").into())
                }
            };

            if buf.ends_with(CRLF) {
                break;
            }
        }

        Ok(buf)
    }
}

enum BodyKind {
    Chunked,
    Empty,
    Length(usize),
}

impl BodyKind {
    fn try_from_headers(headers: &HashMap<String, String>) -> Result<Self> {
        if headers.get("Transfer-Encoding").map(|h| h.as_str()) == Some("chunked") {
            Ok(BodyKind::Chunked)
        } else if let Some(length_s) = headers.get("Content-Length") {
            let length = length_s
                .parse()
                .map_err::<Error, _>(|_| ContentLength.into())?;
            Ok(BodyKind::Length(length))
        } else {
            Ok(BodyKind::Empty)
        }
    }
}

#[inline]
fn parse_version(bytes: &mut Bytes) -> Result<HttpVersion> {
    if let Some(eight) = bytes.peek_n::<[u8; 8]>() {
        let h10 = u64::from_ne_bytes(*b"HTTP/1.O");
        let h11 = u64::from_ne_bytes(*b"HTTP/1.1");

        // SAFETY: The bytes are guaranteed to be 8 bytes long.
        unsafe {
            bytes.advance(8);
        }

        let version = u64::from_ne_bytes(eight);

        return match version {
            v if v == h10 => Ok(HttpVersion::Http1_0),
            v if v == h11 => Ok(HttpVersion::Http1_1),
            _ => Err(Version.into()),
        };
    }

    Err(Version.into())
}

#[inline]
fn parse_status(bytes: &mut Bytes<'_>) -> Result<u16> {
    let hundreds = expect!(bytes.next() == b'0'..=b'9' => Err(Status.into()));
    let tens = expect!(bytes.next() == b'0'..=b'9' => Err(Status.into()));
    let ones = expect!(bytes.next() == b'0'..=b'9' => Err(Status.into()));

    Ok((hundreds - b'0') as u16 * 100 + (tens - b'0') as u16 * 10 + (ones - b'0') as u16)
}

#[inline]
fn parse_reason<'buf>(bytes: &mut Bytes<'buf>) -> Result<&'buf str> {
    let mut seen_obs_text = false;
    loop {
        let b = next!(bytes => Err(Reason.into()));
        if b == b'\r' {
            expect!(bytes.next() == b'\n' => Err(Reason.into()));
            return Ok(
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
            );
        } else if b == b'\n' {
            return Ok(
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
            );
        } else if !(b == 0x09 || b == b' ' || (0x21..=0x7E).contains(&b) || b >= 0x80) {
            return Err(Reason.into());
        } else if b >= 0x80 {
            seen_obs_text = true;
        }
    }
}

#[inline]
fn parse_headers(bytes: &mut Bytes, map: &mut HashMap<String, String>) -> Result<usize> {
    let start = bytes.as_ref().as_ptr() as usize;

    loop {
        match next!(bytes => Err(Header.into())) {
            b'\r' => {
                expect!(bytes.next() == b'\n' => Err(Header.into()));
                return Ok(bytes.as_ref().as_ptr() as usize - start);
            }
            b'\n' => return Ok(bytes.as_ref().as_ptr() as usize - start),
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
                    match next!(bytes => Err(Header.into())) {
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
fn parse_chunked_body<R: Read>(parser: &mut Parser<R>) -> Result<Vec<u8>> {
    let mut body = Vec::new();

    loop {
        let chunk_length_b = parser.read_next_line()?;
        let chunk_length_s = std::str::from_utf8(&chunk_length_b[..chunk_length_b.len() - 2])
            .map_err::<Error, _>(|_| ChunkSize.into())?;
        let chunk_length =
            usize::from_str_radix(chunk_length_s, 16).map_err::<Error, _>(|_| ChunkSize.into())?;

        if chunk_length == 0 {
            // Read the last CRLF.
            parser
                .read_exact(&mut [0; 2])
                .map_err::<Error, _>(|_| Chunk.into())?;
            break;
        }

        let mut chunk = vec![0; chunk_length];
        parser
            .read_exact(&mut chunk)
            .map_err::<Error, _>(|_| Chunk.into())?;
        body.extend(chunk);

        // Read the CRLF.
        parser.read_exact(&mut [0; 2])?;
    }

    Ok(body)
}

#[inline]
fn parse_length_body<R: Read>(parser: &mut Parser<R>) -> Result<Vec<u8>> {
    let length = match parser.kind {
        BodyKind::Length(length) => length,
        _ => unreachable!("parse_length_body called with non-length body kind"),
    };

    let mut buf = vec![0; length];
    parser.read_exact(&mut buf)?;

    Ok(buf)
}

impl<R> TryFrom<BufReader<R>> for Response
where
    R: Read,
{
    type Error = Error;

    fn try_from(reader: BufReader<R>) -> Result<Self> {
        let mut parser = Parser::new(reader);
        let (version, status, reason, headers) = parser.parse_until_headers()?;
        let body = parser.parse_body()?;

        let body = match (body, status) {
            (Some(_), 204 | 304) => Vec::new(),
            (Some(body), 200..=399) => body,
            (Some(body), 400..=599) => body,
            _ => Vec::new(),
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

impl<R> From<R> for Parser<R>
where
    R: Read,
{
    fn from(inner: R) -> Self {
        Parser::new(inner)
    }
}

impl<R> Read for Parser<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.inner.read(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() -> Result<()> {
        let version = b"HTTP/1.1 200 OK\r\n";
        let mut version = Bytes::new(version);

        let version = parse_version(&mut version)?;

        assert_eq!(version, HttpVersion::Http1_1);

        Ok(())
    }

    #[test]
    fn test_parse_status() -> Result<()> {
        let status = b"200 OK\r\n";
        let mut status = Bytes::new(status);

        let status = parse_status(&mut status)?;

        assert_eq!(status, 200);

        Ok(())
    }

    #[test]
    fn test_parse_reason() -> Result<()> {
        let reason = b"OK\r\n";
        let mut reason = Bytes::new(reason);

        let reason = parse_reason(&mut reason)?;

        assert_eq!(reason, "OK");

        Ok(())
    }

    #[test]
    fn test_parse_headers() -> Result<()> {
        let headers = b"Content-Type: text/plain\r\nContent-Length: 12\r\n\r\n";
        let mut headers = Bytes::new(headers);
        let mut map = HashMap::new();

        let _ = parse_headers(&mut headers, &mut map)?;

        assert_eq!(map.get("Content-Type"), Some(&"text/plain".to_string()));
        assert_eq!(map.get("Content-Length"), Some(&"12".to_string()));

        Ok(())
    }

    #[test]
    fn test_parse_chunked_body() -> Result<()> {
        let bytes: &[u8] = b"4\r\nWiki\r\n7\r\npedia i\r\nB\r\nn \r\nchunks.\r\n0\r\n\r\n";
        let mut parser = Parser {
            inner: BufReader::new(bytes),
            kind: BodyKind::Chunked,
        };

        let body = parse_chunked_body(&mut parser)?;

        assert_eq!(body, b"Wikipedia in \r\nchunks.");

        Ok(())
    }

    #[test]
    fn test_parse_length_body() -> Result<()> {
        let bytes: &[u8] = b"Hello, World!";
        let mut parser = Parser {
            inner: BufReader::new(bytes),
            kind: BodyKind::Length(13),
        };

        let body = parse_length_body(&mut parser)?;

        assert_eq!(body, b"Hello, World!");

        Ok(())
    }

    #[test]
    fn test_parse_respons_with_chunked_body() -> Result<()> {
        let response: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nTransfer-Encoding: chunked\r\n\r\n5\r\n\"Wiki\r\n7\r\npedia i\r\nA\r\nn chunks.\"\r\n0\r\n\r\n";
        let response = Response::try_from(BufReader::new(response))?;

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
        assert_eq!(response.body, b"\"Wikipedia in chunks.\"");

        Ok(())
    }

    #[test]
    fn test_parse_response_with_length_body() -> Result<()> {
        let response: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 15\r\n\r\n\"Hello, World!\"";
        let response = Response::try_from(BufReader::new(response))?;

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
        assert_eq!(response.body, b"\"Hello, World!\"");

        Ok(())
    }

    #[test]
    fn parse_response_with_empty_body() -> Result<()> {
        let response: &[u8] =
            b"HTTP/1.1 204 No-Content\r\nContent-Type: none\r\nVersion: v1.44\r\n\r\n";
        let response = Response::try_from(BufReader::new(response))?;

        assert_eq!(response.version, HttpVersion::Http1_1);
        assert_eq!(response.status, 204);
        assert_eq!(response.reason, "No-Content");
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"none".to_string())
        );
        assert_eq!(response.headers.get("Version"), Some(&"v1.44".to_string()));
        assert!(response.body.is_empty());

        Ok(())
    }

    #[test]
    fn test_convert_response() {
        let response: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 15\r\n\r\n\"Hello, World!\"";
        let response = Response::try_from(BufReader::new(response)).unwrap();

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
        assert_eq!(response.body, b"\"Hello, World!\"");
    }
}
