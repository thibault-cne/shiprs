use std::collections::HashMap;
use std::io::BufReader;
use std::io::Read;
use std::io::{Error as IoError, ErrorKind, Result as IoResult};

use super::iter::Bytes;
use super::{HttpVersion, CRLF};
use crate::error::{Error, HttpParsingErrorKind::*, Result};

const END_OF_HEADERS: &[u8] = b"\r\n\r\n";

pub struct ResponseParser<R> {
    inner: BufReader<R>,
}

impl<R> ResponseParser<R>
where
    R: Read,
{
    pub fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner),
        }
    }

    pub fn into_inner(self) -> BufReader<R> {
        self.inner
    }

    pub fn parse_until_headers(
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

        Ok((version, status, reason, headers))
    }

    fn read_until_headers(&mut self) -> IoResult<Vec<u8>> {
        let mut buf = Vec::new();

        loop {
            dbg!("{}", unsafe { std::str::from_utf8_unchecked(&buf) });
            let byte = self.inner.by_ref().bytes().next();

            match byte {
                Some(b) => buf.push(b?),
                None => return Err(IoError::new(ErrorKind::ConnectionAborted, "Unexpected EOF")),
            };

            if buf.ends_with(END_OF_HEADERS) {
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
    Unsupported,
}

pub struct BodyParser<R> {
    inner: BufReader<R>,
    kind: BodyKind,
}

impl<R> BodyParser<R>
where
    R: Read,
{
    pub fn new(inner: R, headers: &HashMap<String, String>) -> Result<Self> {
        let kind = match headers.get("Transfer-Encoding") {
            Some(value) if value == "chunked" => BodyKind::Chunked,
            _ => match headers.get("Content-Length") {
                Some(value) => match value.parse::<usize>() {
                    Ok(length) => BodyKind::Length(length),
                    Err(_) => return Err(ContentLength.into()),
                },
                None => BodyKind::Empty,
            },
        };

        Ok(Self {
            inner: BufReader::new(inner),
            kind,
        })
    }

    pub fn parse(&mut self) -> Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();
        self.inner.read_to_end(&mut buf)?;
        let mut bytes = Bytes::new(&buf);

        match self.kind {
            BodyKind::Chunked => parse_chunked_body(&mut bytes).map(Some),
            BodyKind::Length(length) => parse_length_body(&mut bytes, length).map(Some),
            BodyKind::Empty => Ok(None),
            BodyKind::Unsupported => Err(UnsupportedBodyEncoding.into()),
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
fn parse_chunked_body(bytes: &mut Bytes) -> Result<Vec<u8>> {
    let mut body = Vec::new();

    loop {
        match next!(bytes => Err(Chunk.into())) {
            b'\r' => {
                expect!(bytes.next() == b'\n' => Err(ChunkSize.into()));
                // SAFETY: (1) calling bytes.slice_skip(2) is safe, because at least two next! calls have been made
                let chunk_size = unsafe { bytes.slice_skip(2) };
                let chunk_size_s = unsafe { std::str::from_utf8_unchecked(chunk_size) };
                let chunk_size = usize::from_str_radix(chunk_size_s, 16)
                    .map_err(|_| Into::<Error>::into(ChunkSize))?;

                if chunk_size == 0 && bytes.peek_n::<[u8; 2]>() == Some(*CRLF) {
                    return Ok(body);
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

#[inline]
fn parse_length_body(bytes: &mut Bytes, length: usize) -> Result<Vec<u8>> {
    if bytes.len() < length {
        return Err(BodyLength.into());
    }

    // SAFETY: it's safe to advance the bytes iterator by `length` bytes, because
    // the length was validated to be a valid length and the bytes iterator is guaranteed
    // to have at least `length` bytes left.
    unsafe { bytes.advance(length) };

    Ok(bytes.slice().to_vec())
}

impl<'buf> From<&'buf [u8]> for ResponseParser<&'buf [u8]> {
    fn from(buf: &'buf [u8]) -> Self {
        ResponseParser {
            inner: BufReader::new(buf),
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
        let bytes = b"4\r\nWiki\r\n7\r\npedia i\r\nB\r\nn \r\nchunks.\r\n0\r\n\r\n";
        let mut bytes = Bytes::new(bytes);
        let body = parse_chunked_body(&mut bytes)?;

        assert_eq!(body, b"Wikipedia in \r\nchunks.");

        Ok(())
    }

    #[test]
    fn test_parse_length_body() -> Result<()> {
        let bytes = b"Hello, World!";
        let mut bytes = Bytes::new(bytes);

        let body = parse_length_body(&mut bytes, 13)?;

        assert_eq!(body, b"Hello, World!");

        Ok(())
    }

    #[test]
    fn test_deserialize_body() -> Result<()> {
        let bytes = b"5\r\n\"Wiki\r\n7\r\npedia i\r\nA\r\nn chunks.\"\r\n0\r\n\r\n";
        let mut bytes = Bytes::new(bytes);

        let body = parse_chunked_body(&mut bytes)?;

        let expected = b"Wikipedia in chunks.";
        let expected_s = unsafe { std::str::from_utf8_unchecked(expected) };

        let parsed_body = serde_json::from_slice::<&str>(&body)?;
        assert_eq!(parsed_body, expected_s);

        Ok(())
    }
}
