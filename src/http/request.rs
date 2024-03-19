use std::collections::HashMap;

use serde::Serialize;

use super::uri::Uri;
use crate::http::Method;

#[derive(Debug, Clone)]
pub struct Request<'a, B>
where
    B: Serialize,
{
    method: Method,
    uri: Uri<'a>,
    headers: HashMap<String, String>,
    body: Option<B>,
}

impl<'a, B> Request<'a, B>
where
    B: Serialize,
{
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

impl<B> std::fmt::Display for Request<'_, B>
where
    B: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");
        write!(
            f,
            "{} {} HTTP/1.1\r\n{}\r\n\r\n{}",
            self.method,
            self.uri.as_ref(),
            headers,
            self.body
                .as_ref()
                .map(|b| serde_json::to_string(&b).unwrap())
                .unwrap_or_default()
        )
    }
}

pub struct RequestBuilder<'a, O, B>
where
    O: Serialize,
    B: Serialize,
{
    method: Method,
    path: &'a str,
    query: Option<O>,
    headers: HashMap<String, String>,
    body: Option<B>,
}

impl<'a, O, B> RequestBuilder<'a, O, B>
where
    O: Serialize,
    B: Serialize,
{
    /// Create a new request builder.
    /// Prefer using the `get`, `post`, `put`, `delete`, and `head` methods
    /// to create the right request.
    fn new<P, M>(method: M, path: P) -> Self
    where
        P: Into<&'a str>,
        M: Into<Method>,
    {
        RequestBuilder {
            method: method.into(),
            path: path.into(),
            query: None,
            headers: HashMap::from([
                ("Host".to_string(), crate::API_VERSION.to_string()),
                ("Content-Type".to_string(), "application/json".to_string()),
            ]),
            body: None,
        }
    }

    pub fn get<P>(path: P) -> Self
    where
        P: Into<&'a str>,
    {
        RequestBuilder::new(Method::Get, path)
    }

    pub fn post<P>(path: P) -> Self
    where
        P: Into<&'a str>,
    {
        RequestBuilder::new(Method::Post, path)
    }

    pub fn put<P>(path: P) -> Self
    where
        P: Into<&'a str>,
    {
        RequestBuilder::new(Method::Put, path)
    }

    pub fn delete<P>(path: P) -> Self
    where
        P: Into<&'a str>,
    {
        RequestBuilder::new(Method::Delete, path)
    }

    pub fn head<P>(path: P) -> Self
    where
        P: Into<&'a str>,
    {
        RequestBuilder::new(Method::Head, path)
    }

    pub fn query(mut self, query: Option<O>) -> Self {
        self.query = query;
        self
    }

    pub(crate) fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn body(mut self, body: B) -> Self {
        self.body = Some(body);
        self
    }

    pub fn body_mut(&mut self) -> &mut Option<B> {
        &mut self.body
    }

    /// Build the request.
    pub fn build(mut self) -> Request<'a, B> {
        let uri = Uri::parse(self.path, self.query).unwrap();
        if self.body.is_some() {
            let body = serde_json::to_string(&self.body).unwrap();
            self.headers
                .insert("Content-Length".to_string(), body.len().to_string());
        }

        Request {
            method: self.method,
            uri,
            headers: self.headers,
            body: self.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_request_uri {
        ($req:expr, $expected:literal) => {{
            let req_build = $req.clone().build();
            let req_uri = req_build.split("\r\n").next();
            assert_eq!(req_uri, Some($expected));
        }};
    }

    #[test]
    fn build_request_no_options() {
        let request = RequestBuilder::<String, ()>::get("/containers/json").build();
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

        let request = RequestBuilder::<String, ()>::post("/containers/create").build();
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
        let request = RequestBuilder::<TestOptions, ()>::get("/containers/json")
            .query(Some(options))
            .build();
        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.uri(), "/containers/json?all=true&limit=10");
        assert_request_uri!(request, "GET /containers/json?all=true&limit=10 HTTP/1.1");
    }

    #[derive(Debug, Serialize, PartialEq, Clone)]
    struct TestBody {
        limit: i32,
        test: String,
    }

    #[test]
    fn build_with_body() {
        let body = TestBody {
            limit: 10,
            test: "test".to_string(),
        };
        let json = serde_json::to_string(&body).unwrap();
        let len = json.len();

        let request = RequestBuilder::<(), TestBody>::post("/containers/create")
            .body(body)
            .build();

        assert_eq!(request.method(), Method::Post);
        assert_eq!(request.uri(), "/containers/create");
        assert_request_uri!(request, "POST /containers/create HTTP/1.1");

        assert_eq!(
            request.headers.get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(
            request.headers.get("Content-Length").unwrap(),
            &len.to_string()
        );

        let end = request
            .build()
            .chars()
            .rev()
            .take(json.len())
            .collect::<String>();
        let expected_end = r#"{"limit":10,"test":"test"}"#.chars().rev().collect::<String>();
        assert_eq!(end, expected_end);
    }
}
