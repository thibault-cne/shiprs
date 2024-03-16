use std::borrow::Cow;

use serde::Serialize;

use crate::error::Result;

#[derive(Debug)]
pub struct Uri<'a> {
    encoded: Cow<'a, str>,
}

impl<'a> Uri<'a> {
    pub fn parse<O>(path: &'a str, query: Option<O>) -> Result<Self>
    where
        O: Serialize,
    {
        let mut uri = Cow::Borrowed(path);
        if let Some(query) = query {
            let query = serde_urlencoded::to_string(query)?;
            uri.to_mut().push('?');
            uri.to_mut().push_str(&query);
        }
        Ok(Uri { encoded: uri })
    }
}

impl<'a> AsRef<str> for Uri<'a> {
    fn as_ref(&self) -> &str {
        &self.encoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_parse() {
        let uri = Uri::parse::<String>("/containers/json", None).unwrap();
        assert_eq!(uri.as_ref(), "/containers/json");

        let uri = Uri::parse("/containers/json", Some(vec![("all", "true")])).unwrap();
        assert_eq!(uri.as_ref(), "/containers/json?all=true");
    }
}
