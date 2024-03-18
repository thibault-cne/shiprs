use std::collections::HashMap;

use serde::Deserialize;

use crate::error::{Error, HttpParsingErrorKind::*, Result};
use crate::http::{CR, LF};

// This implementation of HTTP response parsing is mostly taken from
// https://github.com/fristonio/docker.rs
// with minor changes.

/// An HTTP response.
#[derive(Debug, Default)]
pub struct Response<B> {
    status: u16,
    body: B,
    headers: HashMap<String, String>,
}

impl<B> Response<B> {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> &B {
        &self.body
    }

    pub fn into_body(self) -> B {
        self.body
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

impl<B> Response<B>
where
    for<'de> B: Deserialize<'de>,
{
    pub fn parse_http_response(res: &[u8]) -> Result<Response<B>> {
        let mut pos: usize = 0;
        for i in 0..(res.len() - 1) {
            if res[i] == CR && res[i + 1] == LF && res[i + 2] == CR && res[i + 3] == LF {
                pos = i + 3;
                break;
            }
        }

        if pos == 0 {
            return Err(Response.into());
        }

        let (resp_header, resp_body): (&[u8], &[u8]) = res.split_at(pos);

        let header_info = match String::from_utf8(resp_header.to_vec()) {
            Ok(h) => h,
            Err(_) => return Err(Header.into()),
        };

        let body = resp_body[1..].to_owned();

        let mut header_vec: Vec<&str> = header_info.split("\r\n").collect();
        let status = header_vec[0].to_owned();
        let status_vec: Vec<&str> = status.splitn(3, ' ').collect();

        let status: u16 = match status_vec[1].parse() {
            Ok(s) => s,
            Err(_) => return Err(Status.into()),
        };

        header_vec.remove(0);
        let len = header_vec.len();
        header_vec.remove(len - 1);

        let mut headers: HashMap<String, String> = HashMap::new();
        for header in header_vec {
            let item = header.to_owned();
            let item_vec: Vec<&str> = item.splitn(2, ": ").collect();
            headers.insert(item_vec[0].to_owned(), item_vec[1].to_owned());
        }

        let body = match headers.get("Transfer-Encoding") {
            Some(enc) => {
                if enc == "chunked" && !body.is_empty() {
                    Response::<B>::parse_chunk(&body)?
                } else {
                    body
                }
            }
            None => body,
        };

        match status {
            400..=500 => {
                let err = serde_json::from_slice::<shiprs_models::models::ErrorResponse>(&body)?;
                Err(err.into())
            }
            200..=299 => Ok(Response {
                status,
                body: serde_json::from_slice(&body)?,
                headers,
            }),
            _ => Err(Status.into()),
        }
    }

    /// A helper function to parse_http_reseponse, when the Header Transfer-Encoding
    /// `chunked` is present in the response.
    fn parse_chunk(body: &[u8]) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        let mut count: usize = 0;

        loop {
            let mut pos: usize = 0;
            for i in count..body.len() - 1 {
                if body[i] == CR && body[i + 1] == LF {
                    pos = i;
                    break;
                }
            }
            if pos == 0 {
                return Err(ChunkSize.into());
            }

            let size_s = match std::str::from_utf8(&body[count..pos]) {
                Ok(s) => s,
                Err(_) => return Err(Chunk.into()),
            };

            count = pos + 2;
            let size: usize = match usize::from_str_radix(size_s, 16) {
                Ok(s) => s,
                Err(_) => return Err(Chunk.into()),
            };

            if size == 0 && count + 2 == body.len() {
                return Ok(buf);
            }

            buf.extend_from_slice(&body[pos + 2..pos + 2 + size]);
            count = count + size + 2;
        }
    }
}

impl<R> TryFrom<&[u8]> for Response<R>
where
    for<'de> R: Deserialize<'de>,
{
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        Response::parse_http_response(value)
    }
}
