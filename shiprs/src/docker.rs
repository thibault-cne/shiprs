use std::env;

use serde::Serialize;

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

    pub(crate) fn request<S>(&self, req: shiprs_http::Request<S>) -> Result<shiprs_http::Response>
    where
        S: Serialize,
    {
        let res = self.transport.request(req)?;

        match res.status() {
            200..=399 => Ok(res),
            400..=599 => {
                let status = res.status();
                let err = serde_json::from_slice(res.body())?;
                Err(crate::error::Error::docker_api_response(status, err))
            }
            _ => unreachable!("unexpected status code: {}", res.status()),
        }
    }

    pub(crate) fn process_into_value<S, T>(&self, req: shiprs_http::Request<S>) -> Result<T>
    where
        S: Serialize,
        T: for<'de> serde::Deserialize<'de>,
    {
        let res = self.request(req)?;
        serde_json::from_slice(res.body()).map_err(Into::into)
    }

    pub(crate) fn process_into_unit<S>(&self, req: shiprs_http::Request<S>) -> Result<()>
    where
        S: Serialize,
    {
        let _ = self.request(req)?;
        Ok(())
    }

    pub fn containers(&self) -> crate::container::Containers {
        crate::container::Containers::new(self)
    }
}
