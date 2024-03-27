#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Get => f.write_str("GET"),
            Method::Post => f.write_str("POST"),
            Method::Put => f.write_str("PUT"),
            Method::Delete => f.write_str("DELETE"),
            Method::Head => f.write_str("HEAD"),
        }
    }
}
