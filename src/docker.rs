use std::env;

use serde::Serialize;
use shiprs_http::Response;
use shiprs_models::models::ErrorResponse;

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

#[derive(Debug)]
pub enum DockerResponse<T> {
    Success(Response<T>),
    Error(Response<ErrorResponse>),
    Empty(Response<()>),
}

impl<T> DockerResponse<T> {
    pub fn is_success(&self) -> bool {
        matches!(self, DockerResponse::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, DockerResponse::Error(_))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, DockerResponse::Empty(_))
    }

    pub fn success(self) -> Option<Response<T>> {
        match self {
            DockerResponse::Success(response) => Some(response),
            _ => None,
        }
    }

    pub fn success_ref(&self) -> Option<&Response<T>> {
        match self {
            DockerResponse::Success(response) => Some(response),
            _ => None,
        }
    }

    pub fn error(self) -> Option<Response<ErrorResponse>> {
        match self {
            DockerResponse::Error(response) => Some(response),
            _ => None,
        }
    }

    pub fn empty(self) -> Option<Response<()>> {
        match self {
            DockerResponse::Empty(response) => Some(response),
            _ => None,
        }
    }
}

impl<T> From<Response<T>> for DockerResponse<T> {
    fn from(response: Response<T>) -> Self {
        let status = response.status();

        match status {
            204 | 304 => DockerResponse::Empty(response.into_response()),
            200..=399 => DockerResponse::Success(response.into_response()),
            400..=599 => DockerResponse::Error(response.into_response()),
            _ => DockerResponse::Empty(response.into_response()),
        }
    }
}
