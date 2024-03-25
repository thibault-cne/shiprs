#![allow(dead_code)]

use std::io::{BufReader, Write};
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
const END: &[u8] = b"\r\n\r\n";

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

        Response::<R>::try_from(BufReader::new(socket))
    }
}
