use crate::{http::Method, option::OptionIter};

pub struct Request {
    method: Method,
    path: String,
    query: Option<String>,
}

impl Request {
    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    /// Build the request.
    pub fn build(self) -> String {
        format!(
            "{} {}{} HTTP/1.1\r\nHost: {}\r\n\r\n",
            self.method,
            self.path,
            self.query.map(|q| format!("?{}", q)).unwrap_or_default(),
            crate::API_VERSION
        )
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.build().into_bytes()
    }
}

pub struct RequestBuilder {
    method: Method,
    path: String,
    query: Option<String>,
}

impl RequestBuilder {
    /// Create a new request builder.
    /// Prefer using the `get`, `post`, `put`, `delete`, and `head` methods
    /// to create the right request.
    pub fn new<M: Into<Method>, P: Into<String>>(method: M, path: P) -> Self {
        RequestBuilder {
            method: method.into(),
            path: path.into(),
            query: None,
        }
    }

    pub fn get<P: Into<String>>(path: P) -> Self {
        RequestBuilder::new(Method::Get, path)
    }

    pub fn post<P: Into<String>>(path: P) -> Self {
        RequestBuilder::new(Method::Post, path)
    }

    pub fn put<P: Into<String>>(path: P) -> Self {
        RequestBuilder::new(Method::Put, path)
    }

    pub fn delete<P: Into<String>>(path: P) -> Self {
        RequestBuilder::new(Method::Delete, path)
    }

    pub fn head<P: Into<String>>(path: P) -> Self {
        RequestBuilder::new(Method::Head, path)
    }

    /// Add a query parameter to the request.
    pub fn query<S: Into<String>>(mut self, key: S, value: S) -> Self {
        match self.query {
            Some(q) => {
                self.query = Some(format!("{}&{}={}", q, key.into(), value.into()));
            }
            None => {
                self.query = Some(format!("{}={}", key.into(), value.into()));
            }
        }
        self
    }

    pub fn extend_query<S: Into<String>>(mut self, query: S) -> Self {
        match self.query {
            Some(q) => {
                self.query = Some(format!("{}&{}", q, query.into()));
            }
            None => {
                self.query = Some(query.into());
            }
        }
        self
    }

    pub fn extend_query_with_options<O>(self, options: Option<O>) -> Self
    where
        O: IntoIterator<IntoIter = OptionIter>,
    {
        options
            .map(|o| o.into_iter())
            .unwrap_or_default()
            .fold(self, |builder, (k, v)| builder.query(k, v))
    }

    /// Build the request.
    pub fn build(self) -> Request {
        Request {
            method: self.method,
            path: self.path,
            query: self.query,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    macro_rules! request_build {
        ($expected:literal) => {
            format!("{}Host: {}\r\n\r\n", $expected, crate::API_VERSION)
        };
    }

    #[test]
    fn build_request() {
        let request = RequestBuilder::get("/containers/json").build();
        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.path(), "/containers/json");
        assert_eq!(request.query(), None);
        assert_eq!(
            request.build(),
            request_build!("GET /containers/json HTTP/1.1\r\n")
        );

        let request = RequestBuilder::get("/containers/json")
            .query("all", "true")
            .query("limit", "10")
            .build();
        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.path(), "/containers/json");
        assert_eq!(request.query(), Some("all=true&limit=10"));
        assert_eq!(
            request.build(),
            request_build!("GET /containers/json?all=true&limit=10 HTTP/1.1\r\n")
        );

        let request = RequestBuilder::post("/containers/create").build();
        assert_eq!(request.method(), Method::Post);
        assert_eq!(request.path(), "/containers/create");
        assert_eq!(request.query(), None);
        assert_eq!(
            request.build(),
            request_build!("POST /containers/create HTTP/1.1\r\n")
        );
    }

    struct TestOptions {
        all: bool,
        limit: u32,
    }

    impl IntoIterator for TestOptions {
        type Item = (String, String);
        type IntoIter = OptionIter;

        fn into_iter(self) -> Self::IntoIter {
            let options = vec![
                ("all".to_string(), self.all.to_string()),
                ("limit".to_string(), self.limit.to_string()),
            ];
            OptionIter::new(HashMap::from_iter(options))
        }
    }

    #[test]
    fn extend_query_with_options() {
        let options = TestOptions {
            all: true,
            limit: 10,
        };
        let request = RequestBuilder::get("/containers/json")
            .extend_query_with_options(Some(options))
            .build();
        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.path(), "/containers/json");
        assert!(request.query().is_some());
        let query = request.query().unwrap();
        assert!(query == "all=true&limit=10" || query == "limit=10&all=true");
        let request = request.build();
        assert!(
            request == request_build!("GET /containers/json?all=true&limit=10 HTTP/1.1\r\n")
                || request == request_build!("GET /containers/json?limit=10&all=true HTTP/1.1\r\n")
        );
    }
}
