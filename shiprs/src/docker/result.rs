use shiprs_http::Response;
use shiprs_models::models::ErrorResponse;

use crate::error::Result;

/// This type represents the result of a Docker API call.
pub type DockerResult<T> = Result<DockerResponse<Response<T>, Response<ErrorResponse>>>;

/// This enum represents the possible responses from the Docker API.
/// It can be a success, or an error.
///
/// It's a [`Result`] like type. This type is used to represent the result of a Docker API call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DockerResponse<T, E> {
    Success(T),
    Failure(E),
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
    #[inline]
    pub fn is_success(&self) -> bool {
        matches!(self, DockerResponse::Success(_))
    }

    /// Returns true if the response is an Failure.
    ///
    /// # Example
    /// ```
    /// use shiprs::DockerResponse;
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Success("hello");
    /// assert!(!resp.is_Failure());
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Failure("world");
    /// assert!(resp.is_failure());
    /// ```
    #[inline]
    pub fn is_failure(&self) -> bool {
        matches!(self, DockerResponse::Failure(_))
    }

    /// Converts from `DockerResponse<T>` to [`Option<Response<T>>`].
    ///
    /// Converts `self` into an [`Option<Response<T>>`], consuming `self`,
    /// and discarding the failure value, if any.
    ///
    /// # Example
    /// ```
    /// use shiprs::DockerResponse;
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Success("hello");
    /// assert_eq!(resp.success(), Some("hello"));
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Failure("world");
    /// assert_eq!(resp.success(), None);
    /// ```
    #[inline]
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
    /// assert_eq!(resp.failure(), None);
    ///
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Failure("world");
    /// assert_eq!(resp.failure(), Some("world"));
    /// ```
    #[inline]
    pub fn failure(self) -> Option<E> {
        match self {
            DockerResponse::Failure(response) => Some(response),
            _ => None,
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
    /// let resp: DockerResponse<&str, &str> = DockerResponse::Failure("world");
    /// assert_eq!(resp.as_ref(), DockerResponse::Failure(&"world"));
    /// ```
    #[inline]
    pub const fn as_ref(&self) -> DockerResponse<&T, &E> {
        match self {
            DockerResponse::Success(ref response) => DockerResponse::Success(response),
            DockerResponse::Failure(ref response) => DockerResponse::Failure(response),
        }
    }

    /// Converts from `&mut DockerResponse<T, E>` to `DockerResponse<&mut T, &mut E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use shiprs::docker::{DockerResponse, DockerResponse::*};
    ///
    /// fn mutate(r: &mut DockerResponse<i32, i32>) {
    ///     match r.as_mut() {
    ///         Success(v) => *v = 42,
    ///         Failure(e) => *e = 0,
    ///     }
    /// }
    ///
    /// let mut x: DockerResponse<i32, i32> = Success(2);
    /// mutate(&mut x);
    /// assert_eq!(x.unwrap(), 42);
    ///
    /// let mut x: DockerResponse<i32, i32> = Failure(13);
    /// mutate(&mut x);
    /// assert_eq!(x.unwrap_err(), 0);
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> DockerResponse<&mut T, &mut E> {
        match self {
            DockerResponse::Success(ref mut response) => DockerResponse::Success(response),
            DockerResponse::Failure(ref mut response) => DockerResponse::Failure(response),
        }
    }

    /// Returns the contained [`DockerResponse::Success`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`DockerResponse::Failure`]
    /// case explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`DockerResponse::Failure`], with a panic message provided by the
    /// [`DockerResponse::Failure`]'s value.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use shiprs::docker::{DockerResponse, DockerResponse::*};
    ///
    /// let x: DockerResponse<u32, &str> = Success(2);
    /// assert_eq!(x.unwrap(), 2);
    /// ```
    ///
    /// ```should_panic
    /// use shiprs::docker::{DockerResponse, DockerResponse::*};
    ///
    /// let x: DockerResponse<u32, &str> = Failure("emergency failure");
    /// x.unwrap(); // panics with `emergency failure`
    /// ```
    #[inline]
    pub fn unwrap(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self {
            DockerResponse::Success(val) => val,
            DockerResponse::Failure(err) => panic!("{:?}", err),
        }
    }

    /// Returns the contained [`DockerResponse::Failure`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`DockerResponse::Success`], with a custom panic message provided
    /// by the [`DockerResponse::Success`]'s value.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use shiprs::docker::{DockerResponse, DockerResponse::*};
    ///
    /// let x: DockerResponse<u32, &str> = Success(2);
    /// x.unwrap_err(); // panics with `2`
    /// ```
    ///
    /// ```
    /// use shiprs::docker::{DockerResponse, DockerResponse::*};
    ///
    /// let x: DockerResponse<u32, &str> = Failure("emergency failure");
    /// assert_eq!(x.unwrap_failure(), "emergency failure");
    /// ```
    #[inline]
    pub fn unwrap_failure(self) -> E
    where
        T: std::fmt::Debug,
    {
        match self {
            DockerResponse::Success(val) => panic!("{:?}", val),
            DockerResponse::Failure(err) => err,
        }
    }

    /// Returns the contained [`DockerResponse::Success`] value, consuming the `self` value,
    /// without checking that the value is not an [`DockerResponse::Failure`].
    ///
    /// # Safety
    ///
    /// Calling this method on an [`DockerResponse::Failure`] is *[undefined behavior]*.
    ///
    /// # Examples
    ///
    /// ```
    /// use shiprs::docker::{DockerResponse, DockerResponse::*};
    ///
    /// let x: DockerResponse<u32, &str> = Success(2);
    /// assert_eq!(unsafe { x.unwrap_unchecked() }, 2);
    /// ```
    ///
    /// ```no_run
    /// use shiprs::docker::{DockerResponse, DockerResponse::*};
    ///
    /// let x: DockerResponse<u32, &str> = Failure("emergency failure");
    /// unsafe { x.unwrap_unchecked(); } // Undefined behavior!
    /// ```
    #[inline]
    pub unsafe fn unwrap_unchecked(self) -> T {
        debug_assert!(self.is_success());
        match self {
            DockerResponse::Success(val) => val,
            DockerResponse::Failure(_) => std::hint::unreachable_unchecked(),
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
            DockerResponse::Failure(response) => response
                .body()
                .map(|body| DockerResponse::Failure(body))
                .map_err(Into::into),
        }
    }
}

impl<T> From<Response<T>> for DockerResponse<Response<T>, Response<ErrorResponse>> {
    fn from(response: Response<T>) -> Self {
        let status = response.status();

        match status {
            200..=399 => DockerResponse::Success(response.into_response()),
            400..=599 => DockerResponse::Failure(response.into_response()),
            _ => unimplemented!("Unexpected response status"),
        }
    }
}
