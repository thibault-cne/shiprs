use std::env;

use serde::Serialize;

use crate::error::Result;
use crate::transport::Transport;

pub mod result;
pub use result::{DockerResponse, DockerResult};

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

    pub(crate) fn request<S, T>(
        &self,
        req: shiprs_http::Request<S>,
    ) -> Result<shiprs_http::Response<T>>
    where
        S: Serialize,
        T: for<'de> serde::Deserialize<'de>,
    {
        self.transport.request(req)
    }

    pub fn containers(&self) -> crate::container::Containers {
        crate::container::Containers::new(self)
    }
}
