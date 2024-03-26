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

/// This type represents the result of a Docker API call.
pub(crate) type DockerResult<T> = Result<DockerResponse<Response<T>, Response<ErrorResponse>>>;

/// This enum represents the possible responses from the Docker API.
/// It can be a success, or an error. It's a [`Result`] like type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DockerResponse<T, E> {
    Success(T),
    Error(E),
}

impl<T, E> DockerResponse<T, E> {
    /// Returns true if the response is a success.
    ///
    /// # Example
    /// ```
    /// use shiprs::DockerResponse;
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Success("hello");
    /// assert!(resp.is_success());
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Error("world");
    /// assert!(!resp.is_success());
    /// ```
    pub fn is_success(&self) -> bool {
        matches!(self, DockerResponse::Success(_))
    }

    /// Returns true if the response is an error.
    ///
    /// # Example
    /// ```
    /// use shiprs::DockerResponse;
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Success("hello");
    /// assert!(!resp.is_error());
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Error("world");
    /// assert!(resp.is_error());
    /// ```
    pub fn is_error(&self) -> bool {
        matches!(self, DockerResponse::Error(_))
    }

    /// Converts from `DockerResponse<T>` to [`Option<Response<T>>`].
    ///
    /// Converts `self` into an [`Option<Response<T>>`], consuming `self`,
    /// and discarding the error value, if any.
    ///
    /// # Example
    /// ```
    /// use shiprs::DockerResponse;
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Success("hello");
    /// assert_eq!(resp.success(), Some("hello"));
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Error("world");
    /// assert_eq!(resp.success(), None);
    /// ```
    pub fn success(self) -> Option<T> {
        match self {
            DockerResponse::Success(response) => Some(response),
            _ => None,
        }
    }

    /// Converts from `DockerResponse<T>` to [`Option<Response<ErrorResponse>>`].
    ///
    /// Converts `self` into an [`Option<Response<ErrorResponse>>`], consuming `self`,
    /// and discarding the success value, if any.
    ///
    /// # Example
    /// ```
    /// use shiprs::DockerResponse;
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Success("hello");
    /// assert_eq!(resp.error(), None);
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Error("world");
    /// assert_eq!(resp.error(), Some("world"));
    /// ```
    pub fn error(self) -> Option<E> {
        match self {
            DockerResponse::Error(response) => Some(response),
            _ => None,
        }
    }

    /// Converts from `DockerResponse<T, E>` to [`Option<T>`].
    ///
    /// Converts `self` into an [`Option<T>`], consuming `self`,
    /// and discarding the error value, if any.
    ///
    /// # Example
    /// ```
    /// use shiprs::DockerResponse;
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Success("hello");
    /// assert_eq!(resp.ok(), Some("hello"));
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Error("world");
    /// assert_eq!(resp.ok(), None);
    /// ```
    pub fn ok(self) -> Option<T> {
        match self {
            DockerResponse::Success(response) => Some(response),
            DockerResponse::Error(_) => None,
        }
    }

    /// Converts from `&DockerResponse<T>` to `DockerResponse<&T>`.
    ///
    /// Produces a new `DockerResponse`, containing a reference into the original,
    /// leaving the original in place.
    ///
    /// # Example
    /// ```
    /// use shiprs::DockerResponse;
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Success("hello");
    /// assert_eq!(resp.as_ref(), DockerResponse::Success(&"hello"));
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Error("world");
    /// assert_eq!(resp.as_ref(), DockerResponse::Error(&"world"));
    /// ```
    #[inline]
    pub const fn as_ref(&self) -> DockerResponse<&T, &E> {
        match self {
            DockerResponse::Success(ref response) => DockerResponse::Success(response),
            DockerResponse::Error(ref response) => DockerResponse::Error(response),
        }
    }
}

impl<T> DockerResponse<Response<T>, Response<ErrorResponse>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    /// Maps a `DockerResponse<Response<T>, Response<ErrorResponse>>` to `DockerResponse<T, ErrorResponse>`.
    /// This is used to have direct access to the body of the response.
    pub fn body(self) -> Result<DockerResponse<T, ErrorResponse>> {
        match self {
            DockerResponse::Success(response) => response
                .body()
                .map(|body| DockerResponse::Success(body))
                .map_err(Into::into),
            DockerResponse::Error(response) => response
                .body()
                .map(|body| DockerResponse::Error(body))
                .map_err(Into::into),
        }
    }
}

impl<T> From<Response<T>> for DockerResponse<Response<T>, Response<ErrorResponse>> {
    fn from(response: Response<T>) -> Self {
        let status = response.status();

        match status {
            200..=399 => DockerResponse::Success(response.into_response()),
            400..=599 => DockerResponse::Error(response.into_response()),
            _ => unimplemented!("Unexpected response status"),
        }
    }
}
