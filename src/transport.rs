#![allow(dead_code)]

use std::io::{BufReader, Write};
use std::os::unix::net::UnixStream;

use serde::{Deserialize, Serialize};
use shiprs_http::{Request, Response};

use crate::error::Result;

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

    pub(crate) fn request<S, T>(&self, req: Request<S>) -> Result<Response<T>>
    where
        S: Serialize,
        T: for<'de> Deserialize<'de>,
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
    fn request<S, T>(&self, req: Request<S>) -> Result<Response<T>>
    where
        S: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let mut socket = self.socket.try_clone()?;

        let buf = req.into_bytes();
        socket.write_all(&buf)?;

        Response::<T>::try_from(BufReader::new(socket)).map_err(Into::into)
    }
}
