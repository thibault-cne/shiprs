use std::env;

use serde::Deserialize;

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

    pub(crate) fn request<B>(
        &self,
        req: crate::http::request::Request,
    ) -> Result<crate::http::response::Response<B>>
    where
        for<'de> B: Deserialize<'de>,
    {
        self.transport.request(req)
    }

    pub fn containers(&self) -> crate::container::Containers {
        crate::container::Containers::new(self)
    }
}
