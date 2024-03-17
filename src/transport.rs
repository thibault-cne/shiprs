#![allow(dead_code)]

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    http::{request::Request, response::Response},
};

pub(crate) enum Transport {
    Unix {
        client: Client<UnixStream>,
        path: String,
    },
}

impl Transport {
    pub(crate) fn unix<S: Into<String>>(socket: S) -> Result<Self> {
        let socket = socket.into();
        Ok(Transport::Unix {
            client: Client {
                socket: UnixStream::connect(&socket)?,
            },
            path: socket,
        })
    }

    pub(crate) fn request<B, R>(&self, req: Request<B>) -> Result<Response<R>>
    where
        B: Serialize,
        for<'de> R: Deserialize<'de>,
    {
        match self {
            Transport::Unix { client, .. } => client.request(req),
        }
    }
}

const BUFFER_SIZE: usize = 1024;
const CRLF: &[u8] = b"\r\n";
const END: &[u8] = b"0\r\n\r\n";

pub(crate) struct Client<S> {
    socket: S,
}

impl Client<UnixStream> {
    fn request<B, R>(&self, req: Request<B>) -> Result<Response<R>>
    where
        B: Serialize,
        for<'de> R: Deserialize<'de>,
    {
        let mut socket = self.socket.try_clone()?;

        let buf = req.into_bytes();
        socket.write_all(&buf)?;

        let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut raw_resp = Vec::new();
        loop {
            let n = socket.read(&mut buf[..])?;

            buf.iter().take(n).for_each(|b| raw_resp.push(*b));

            if n > 4 && check_buf_end(&buf, n, END) {
                break;
            }
            if n > 1 && check_buf_end(&buf, n, CRLF) {
                continue;
            }
            if n < BUFFER_SIZE {
                break;
            }
        }

        Response::<R>::try_from(raw_resp.as_slice())
    }
}

fn check_buf_end(buf: &[u8], len: usize, needle: &[u8]) -> bool {
    let needle_len = needle.len();
    len >= needle_len && &buf[len - needle_len..len] == needle
}
