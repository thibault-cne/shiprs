use std::env;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::transport::Transport;

pub struct Docker {
    transport: Transport,
}

impl Docker {
    pub fn new() -> Result<Self> {
        match env::var("DOCKER_HOST").ok() {
            Some(host) => {
                #[cfg(feature = "unix-socket")]
                if let Some(socket) = host.strip_prefix("unix://") {
                    return Docker::unix(socket);
                }
                panic!("Only unix sockets are supported yet");
            }
            #[cfg(feature = "unix-socket")]
            None => Docker::unix("/var/run/docker.sock"),
            #[cfg(not(feature = "unix-socket"))]
            None => {
                panic!("Only unix sockets are supported yet");
            }
        }
    }

    #[cfg(feature = "unix-socket")]
    pub(crate) fn unix<S: Into<String>>(socket: S) -> Result<Self> {
        Ok(Docker {
            transport: Transport::unix(socket)?,
        })
    }

    pub(crate) fn request<B, R>(
        &self,
        req: crate::http::request::Request<B>,
    ) -> Result<crate::http::response::Response<R>>
    where
        B: Serialize,
        for<'de> R: Deserialize<'de>,
    {
        self.transport.request(req)
    }

    pub fn containers(&self) -> crate::container::Containers {
        crate::container::Containers::new(self)
    }
}
