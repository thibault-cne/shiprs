use std::collections::HashMap;

use crate::http::promise::Promise;
use crate::models::container::*;
use crate::{docker::Docker, error::Result, option::OptionIter};

/// Interface for interacting with a container.
///
/// # Example
///
/// ```no_run
/// # use shiprs::error::Result;
/// use shiprs::Docker;
///
/// # fn main() -> Result<()> {
/// let docker = Docker::new().unwrap();
/// let container = docker
///     .containers()
///     .get("insert container id here")
///     .inspect(None)?;
/// println!("{:?}", container);
/// # Ok(())
/// # }
pub struct Container<'docker> {
    docker: &'docker Docker,
    id: String,
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
    /// # Parameters
    /// - `options`: ContainerInspectOptions, used to provide options to the inspect method.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let container = docker
    ///    .containers()
    ///    .get("insert container id here")
    ///    .inspect(None)?;
    /// println!("{:?}", container);
    /// # Ok(())
    /// # }
    pub fn inspect(&self, options: Option<ContainerInspectOptions>) -> Result<ContainerDetails> {
        Ok(
            Promise::new(self.docker, format!("/containers/{}/json", self.id))
                .options(options)
                .run()?
                .into_body(),
        )
    }

    /// Retrieves the logs of the docker container.
    /// This corresponds to the `GET /containers/(id)/logs` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerLogs) for more information.
    ///
    /// # Parameters
    /// - `options`: ContainerLogsOptions, used to provide options to the logs method.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let logs = docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .logs(None)?;
    /// println!("{}", logs);
    /// # Ok(())
    /// # }
    pub fn logs(&self, options: Option<ContainerLogsOptions>) -> Result<String> {
        Ok(
            Promise::new(self.docker, format!("/containers/{}/logs", self.id))
                .options(options)
                .run()?
                .into_body(),
        )
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
    /// # Parameters
    /// - `option`: ContainersListOptions, used to provide options to the list method.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let containers = docker
    ///     .containers()
    ///     .list(None)?;
    /// println!("{:?}", containers);
    /// # Ok(())
    /// # }
    pub fn list(&self, option: Option<ContainersListOptions>) -> Result<Vec<ContainerInfo>> {
        Ok(Promise::new(self.docker, "/containers/json")
            .options(option)
            .run()?
            .into_body())
    }

    /// Get a container by id.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let container = docker
    ///     .containers()
    ///     .get("insert container id here");
    /// # Ok(())
    /// # }
    pub fn get<S: Into<String>>(&self, id: S) -> Container {
        Container::new(self.docker, id)
    }
}

/// Options for the `inspect` method.
/// This struct corresponds to the param options of the `GET /containers/(id)/json` endpoint.
/// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerInspect) for more information.
#[derive(Default)]
pub struct ContainerInspectOptions {
    pub size: bool,
}

impl From<bool> for ContainerInspectOptions {
    fn from(size: bool) -> Self {
        ContainerInspectOptions { size }
    }
}

impl IntoIterator for ContainerInspectOptions {
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

/// Options for the `list` method.
/// This struct corresponds to the param options of the `GET /containers/json` endpoint.
/// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerList) for more information.
pub struct ContainersListOptions {
    all: bool,
    limit: i32,
    size: bool,
}

impl ContainersListOptions {
    pub fn new(all: bool, limit: i32, size: bool) -> Self {
        ContainersListOptions { all, limit, size }
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

impl Default for ContainersListOptions {
    fn default() -> Self {
        ContainersListOptions {
            all: false,
            limit: 10,
            size: false,
        }
    }
}

impl From<(bool, i32, bool)> for ContainersListOptions {
    fn from(options: (bool, i32, bool)) -> Self {
        ContainersListOptions {
            all: options.0,
            limit: options.1,
            size: options.2,
        }
    }
}

impl From<HashMap<String, String>> for ContainersListOptions {
    fn from(options: HashMap<String, String>) -> Self {
        let all = options.get("all").map_or(false, |v| v == "true");
        let limit = options.get("limit").map_or(10, |v| v.parse().unwrap_or(10));
        let size = options.get("size").map_or(false, |v| v == "true");
        ContainersListOptions { all, limit, size }
    }
}

impl IntoIterator for ContainersListOptions {
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

/// Options for the `logs` method.
/// This struct corresponds to the param options of the `GET /containers/(id)/logs` endpoint.
/// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerLogs) for more information.
pub struct ContainerLogsOptions {
    follow: bool,
    stdout: bool,
    stderr: bool,
    since: i32,
    until: i32,
    timestamps: bool,
    tail: String,
}

impl Default for ContainerLogsOptions {
    fn default() -> Self {
        ContainerLogsOptions {
            follow: false,
            stdout: false,
            stderr: false,
            since: 0,
            until: 0,
            timestamps: false,
            tail: "all".to_string(),
        }
    }
}

impl IntoIterator for ContainerLogsOptions {
    type Item = (String, String);
    type IntoIter = OptionIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut options = HashMap::new();
        if self.follow {
            options.insert("follow".to_string(), "true".to_string());
        }
        if self.stdout {
            options.insert("stdout".to_string(), "true".to_string());
        }
        if self.stderr {
            options.insert("stderr".to_string(), "true".to_string());
        }
        if self.since > 0 {
            options.insert("since".to_string(), self.since.to_string());
        }
        if self.until > 0 {
            options.insert("until".to_string(), self.until.to_string());
        }
        if self.timestamps {
            options.insert("timestamps".to_string(), "true".to_string());
        }
        if self.tail != "all" {
            options.insert("tail".to_string(), self.tail);
        }
        OptionIter::new(options)
    }
}
