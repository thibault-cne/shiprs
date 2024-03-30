pub mod container;
pub mod docker;
pub mod error;
mod image;
mod network;
mod transport;

pub use docker::Docker;

pub(crate) fn serialize_as_json<T: serde::Serialize, S: serde::Serializer>(
    t: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(
        &serde_json::to_string(t).map_err(|e| serde::ser::Error::custom(e.to_string()))?,
    )
}
