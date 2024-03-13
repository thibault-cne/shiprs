use crate::models::container::*;
use crate::{docker::Docker, error::Result, http::request::RequestBuilder, OptionTrait};

pub struct Container<'docker> {
    docker: &'docker Docker,
    id: String,
    options: Option<InspectContainerOptions>,
}

#[derive(Default)]
pub struct InspectContainerOptions {
    pub size: bool,
}

impl<'docker> Container<'docker> {
    pub fn new<S: Into<String>>(docker: &'docker Docker, id: S) -> Self {
        Container {
            docker,
            id: id.into(),
            options: None,
        }
    }

    pub fn inspect(&self) -> Result<ContainerDetails> {
        let request = RequestBuilder::get(format!("/containers/{}/json", self.id))
            .extend_query_with_options(self.options.as_ref())
            .build();

        let resp = self.docker.request(request)?;
        Ok(resp.into_body())
    }

    pub fn options<O: Into<InspectContainerOptions>>(mut self, options: O) -> Self {
        let options = options.into();
        self.options = Some(options);
        self
    }

    pub fn logs(&self) -> Result<String> {
        let request = RequestBuilder::get(format!("/containers/{}/logs", self.id)).build();
        let resp = self.docker.request(request)?;
        Ok(resp.into_body())
    }
}

pub struct Containers<'docker> {
    docker: &'docker Docker,
}

impl<'docker> Containers<'docker> {
    pub fn new(docker: &'docker Docker) -> Self {
        Containers { docker }
    }

    pub fn list(&self) -> Result<Vec<ContainerInfo>> {
        let request = RequestBuilder::get("/containers/json").build();
        let resp = self.docker.request(request)?;
        Ok(resp.into_body())
    }

    pub fn get<S: Into<String>>(&self, id: S) -> Container {
        Container::new(self.docker, id)
    }
}

impl From<bool> for InspectContainerOptions {
    fn from(size: bool) -> Self {
        InspectContainerOptions { size }
    }
}

impl OptionTrait for InspectContainerOptions {
    fn as_string(&self) -> String {
        if self.size {
            "size=true".to_string()
        } else {
            String::new()
        }
    }
}
