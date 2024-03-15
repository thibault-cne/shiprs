use crate::{
    error::Result,
    http::{request::RequestBuilder, response::Response},
    option::OptionIter,
};

pub struct Promise<'docker, O, T> {
    url: String,
    docker: &'docker crate::Docker,
    options: Option<O>,
    ty: std::marker::PhantomData<T>,
}

impl<'docker, O, T> Promise<'docker, O, T>
where
    O: IntoIterator<IntoIter = OptionIter>,
    for<'de> T: serde::Deserialize<'de>,
{
    pub(crate) fn new<S: Into<String>>(docker: &'docker crate::Docker, url: S) -> Self {
        Promise {
            url: url.into(),
            docker,
            options: None,
            ty: std::marker::PhantomData,
        }
    }

    pub fn options(mut self, options: O) -> Self {
        self.options = Some(options);
        self
    }

    pub fn run(self) -> Result<Response<T>> {
        let request = RequestBuilder::get(self.url.clone())
            .extend_query_with_options(self.options)
            .build();
        self.docker.request(request)
    }
}
