pub mod container;
mod docker;
pub mod error;
mod http;
mod image;
mod network;
mod transport;

const API_VERSION: &str = "v1.44";

pub use docker::Docker;

pub(crate) fn serialize_as_json<T: serde::Serialize, S: serde::Serializer>(
    t: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(
        &serde_json::to_string(t).map_err(|e| serde::ser::Error::custom(e.to_string()))?,
    )
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}
