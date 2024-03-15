use std::collections::HashMap;

use crate::http::promise::Promise;
use crate::models::container::*;
use crate::{docker::Docker, error::Result, http::request::RequestBuilder, option::OptionIter};

/// Interface for interacting with a container.
///
/// # Example
///
/// ```should_panic
/// # use shiprs::error::Result;
/// use shiprs::Docker;
///
/// # fn main() -> Result<()> {
/// let docker = Docker::new().unwrap();
/// let container = docker
///     .containers()
///     .get("insert container id here")
///     .inspect()
///     .run();
/// println!("{:?}", container);
/// # Ok(())
/// # }
pub struct Container<'docker> {
    docker: &'docker Docker,
    id: String,
}

#[derive(Default)]
pub struct ContainerOptions {
    pub size: bool,
}

impl<'docker> Container<'docker> {
    pub(crate) fn new<S: Into<String>>(docker: &'docker Docker, id: S) -> Self {
        Container {
            docker,
            id: id.into(),
        }
    }

    /// Inspects the docker container details.
    /// This corresponds to the `GET /containers/(id)/json` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerInspect) for more information.
    ///
    /// # Usage
    /// This returns a `Promise` that will resolve to a `ContainerDetails` struct and using a `ContainerOptions` struct as options.
    ///
    /// # Example
    /// ```should_panic
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let container = docker
    ///    .containers()
    ///    .get("insert container id here")
    ///    .inspect()
    ///    .run();
    /// println!("{:?}", container);
    /// # Ok(())
    /// # }
    pub fn inspect(&self) -> Promise<ContainerOptions, ContainerDetails> {
        Promise::new(self.docker, format!("/containers/{}/json", self.id))
    }

    pub fn logs(&self) -> Result<String> {
        let request = RequestBuilder::get(format!("/containers/{}/logs", self.id)).build();
        let resp = self.docker.request(request)?;
        Ok(resp.into_body())
    }
}

/// Interface for interacting with docker containers.
pub struct Containers<'docker> {
    docker: &'docker Docker,
}

impl<'docker> Containers<'docker> {
    pub(crate) fn new(docker: &'docker Docker) -> Self {
        Containers { docker }
    }

    /// Lists the docker containers.
    /// This corresponds to the `GET /containers/json` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerList) for more information.
    ///
    /// # Usage
    /// This returns a `Promise` that will resolve to a `Vec<ContainerInfo>` struct and using a `ContainersOptions` struct as options.
    ///
    /// # Example
    /// ```should_panic
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let containers = docker
    ///     .containers()
    ///     .list()
    ///     .run();
    /// println!("{:?}", containers);
    /// # panic!("This shlould always panic even if docker is running.");
    /// # Ok(())
    /// # }
    pub fn list(&self) -> Promise<ContainersOptions, Vec<ContainerInfo>> {
        Promise::new(self.docker, "/containers/json")
    }

    pub fn get<S: Into<String>>(&self, id: S) -> Container {
        Container::new(self.docker, id)
    }
}

impl From<bool> for ContainerOptions {
    fn from(size: bool) -> Self {
        ContainerOptions { size }
    }
}

impl IntoIterator for ContainerOptions {
    type Item = (String, String);
    type IntoIter = OptionIter;

    fn into_iter(self) -> Self::IntoIter {
        let options: Vec<(String, String)> = if self.size {
            vec![("size".to_string(), "true".to_string())]
        } else {
            Vec::new()
        };
        OptionIter::new(HashMap::from_iter(options))
    }
}

pub struct ContainersOptions {
    all: bool,
    limit: i32,
    size: bool,
}

impl ContainersOptions {
    pub fn new(all: bool, limit: i32, size: bool) -> Self {
        ContainersOptions { all, limit, size }
    }

    pub fn all(mut self, all: bool) -> Self {
        self.all = all;
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = limit;
        self
    }

    pub fn size(mut self, size: bool) -> Self {
        self.size = size;
        self
    }
}

impl Default for ContainersOptions {
    fn default() -> Self {
        ContainersOptions {
            all: false,
            limit: 10,
            size: false,
        }
    }
}

impl From<(bool, i32, bool)> for ContainersOptions {
    fn from(options: (bool, i32, bool)) -> Self {
        ContainersOptions {
            all: options.0,
            limit: options.1,
            size: options.2,
        }
    }
}

impl From<HashMap<String, String>> for ContainersOptions {
    fn from(options: HashMap<String, String>) -> Self {
        let all = options.get("all").map_or(false, |v| v == "true");
        let limit = options.get("limit").map_or(10, |v| v.parse().unwrap_or(10));
        let size = options.get("size").map_or(false, |v| v == "true");
        ContainersOptions { all, limit, size }
    }
}

impl IntoIterator for ContainersOptions {
    type Item = (String, String);
    type IntoIter = OptionIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut options = HashMap::new();
        if self.all {
            options.insert("all".to_string(), "true".to_string());
        }
        if self.size {
            options.insert("size".to_string(), "true".to_string());
        }
        if self.limit > 0 {
            options.insert("limit".to_string(), self.limit.to_string());
        }
        OptionIter::new(options)
    }
}
