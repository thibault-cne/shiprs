#[macro_use]
mod macros;

mod bytes;
mod error;
mod io;
mod method;
mod request;
mod response;
mod uri;
mod version;

const CRLF: &[u8] = b"\r\n";
const HEADERS_END: &[u8] = b"\r\n\r\n";
const DOCKER_API_VERSION: &str = "v1.44";

pub use error::Error;
pub use request::{Request, RequestBuilder};
pub use response::Response;
