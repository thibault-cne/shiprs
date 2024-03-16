use std::collections::HashMap;

use serde::Serialize;

use super::uri::Uri;
use crate::http::Method;

pub struct Request<'a> {
    method: Method,
    uri: Uri<'a>,
    headers: HashMap<String, String>,
}

impl<'a> Request<'a> {
    pub fn method(&self) -> Method {
        self.method
    }

    pub fn uri(&self) -> &str {
        self.uri.as_ref()
    }

    /// Build the request.
    pub fn build(self) -> String {
        self.to_string()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.build().into_bytes()
    }
}

impl std::fmt::Display for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");
        write!(
            f,
            "{} {} HTTP/1.1\r\n{}\r\n\r\n",
            self.method,
            self.uri.as_ref(),
            headers
        )
    }
}

pub struct RequestBuilder<'a, O>
where
    O: Serialize,
{
    method: Method,
    path: &'a str,
    query: Option<O>,
    headers: HashMap<String, String>,
}

impl<'a, O> RequestBuilder<'a, O>
where
    O: Serialize,
{
    /// Create a new request builder.
    /// Prefer using the `get`, `post`, `put`, `delete`, and `head` methods
    /// to create the right request.
    fn new<M: Into<Method>>(method: M, path: &'a str) -> Self {
        RequestBuilder {
            method: method.into(),
            path,
            query: None,
            headers: HashMap::from([
                ("Host".to_string(), crate::API_VERSION.to_string()),
                ("Content-Type".to_string(), "application/json".to_string()),
            ]),
        }
    }

    pub fn get<P>(path: P) -> Self
    where
        P: Into<&'a str>,
    {
        RequestBuilder::new(Method::Get, path.into())
    }

    pub fn post(path: &'a str) -> Self {
        RequestBuilder::new(Method::Post, path)
    }

    pub fn put(path: &'a str) -> Self {
        RequestBuilder::new(Method::Put, path)
    }

    pub fn delete(path: &'a str) -> Self {
        RequestBuilder::new(Method::Delete, path)
    }

    pub fn head(path: &'a str) -> Self {
        RequestBuilder::new(Method::Head, path)
    }

    pub fn query(mut self, query: Option<O>) -> Self {
        self.query = query;
        self
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Build the request.
    pub fn build(self) -> Request<'a> {
        let uri = Uri::parse(self.path, self.query).unwrap();
        Request {
            method: self.method,
            uri,
            headers: self.headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_request_uri {
        ($req:expr, $expected:literal) => {{
            let req_build = $req.build();
            let req_uri = req_build.split("\r\n").next();
            assert_eq!(req_uri, Some($expected));
        }};
    }

    #[test]
    fn build_request_no_options() {
        let request = RequestBuilder::<String>::get("/containers/json").build();
        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.uri(), "/containers/json");
        assert_eq!(
            request.headers.get("Host"),
            Some(&crate::API_VERSION.to_string())
        );
        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&String::from("application/json"))
        );
        assert_request_uri!(request, "GET /containers/json HTTP/1.1");

        let request = RequestBuilder::<String>::post("/containers/create").build();
        assert_eq!(request.method(), Method::Post);
        assert_eq!(request.uri(), "/containers/create");
        assert_eq!(
            request.headers.get("Host"),
            Some(&crate::API_VERSION.to_string())
        );
        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&String::from("application/json"))
        );
        assert_request_uri!(request, "POST /containers/create HTTP/1.1");
    }

    #[derive(Serialize)]
    struct TestOptions {
        all: bool,
        limit: u32,
    }

    #[test]
    fn build_request_with_options() {
        let options = TestOptions {
            all: true,
            limit: 10,
        };
        let request = RequestBuilder::get("/containers/json")
            .query(Some(options))
            .build();
        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.uri(), "/containers/json?all=true&limit=10");
        assert_request_uri!(request, "GET /containers/json?all=true&limit=10 HTTP/1.1");
    }
}
