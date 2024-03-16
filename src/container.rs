use std::collections::HashMap;

use serde::Serialize;

use crate::http::request::RequestBuilder;
use crate::models::container::*;
use crate::{docker::Docker, error::Result};

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
    pub fn inspect(&self, options: Option<InspectContainerOptions>) -> Result<ContainerDetails> {
        let url = format!("/containers/{}/json", self.id);
        let request = RequestBuilder::get(&*url).query(options).build();
        let response = self.docker.request(request)?;

        Ok(response.into_body())
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
    pub fn logs(&self, options: Option<LogsContainerOptions>) -> Result<String> {
        let url = format!("/containers/{}/logs", self.id);
        let request = RequestBuilder::get(&*url).query(options).build();
        let response = self.docker.request(request)?;

        Ok(response.into_body())
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
    ///     .list::<String>(None)?;
    /// println!("{:?}", containers);
    /// # Ok(())
    /// # }
    pub fn list<T>(&self, options: Option<ListContainersOption<T>>) -> Result<Vec<ContainerInfo>>
    where
        T: Into<String> + std::hash::Hash + Eq + Serialize,
    {
        let url = "/containers/json";
        let request = RequestBuilder::get(url).query(options).build();
        let response = self.docker.request(request)?;

        Ok(response.into_body())
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
#[derive(Default, Serialize)]
pub struct InspectContainerOptions {
    pub size: bool,
}

impl From<bool> for InspectContainerOptions {
    fn from(size: bool) -> Self {
        InspectContainerOptions { size }
    }
}

/// Parameters used for the [List Container API](Containers::list)
///
/// ## Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// use shiprs::container::ListContainersOption;
///
/// // Get all running containers
/// let options = ListContainersOption {
///     all: true,
///     filters: HashMap::from([("status", vec!["running"])]),
///     ..Default::default()
/// };
/// ```
///
/// ```rust
/// use shiprs::container::ListContainersOption;
///
/// // Get all containers
/// let options: ListContainersOption<&str> = ListContainersOption {
///    all: true,
///   ..Default::default()
/// };
/// ```
#[derive(Default, Serialize)]
pub struct ListContainersOption<T>
where
    T: Into<String> + Eq + std::hash::Hash + Serialize,
{
    /// Return all containers. By default, only running containers are shown.
    pub all: bool,
    /// Return this number of most recently created containers, including non-running ones.
    pub limit: Option<isize>,
    /// Return the size of container as fields `SizeRw` and `SizeRootFs`.
    pub size: bool,

    /// Filters to process on the container list, encoded as JSON. Available filters:
    ///  - `ancestor`=`(<image-name>[:<tag>]`, `<image id>`, or `<image@digest>`)
    ///  - `before`=(`<container id>` or `<container name>`)
    ///  - `expose`=(`<port>[/<proto>]`|`<startport-endport>`/`[<proto>]`)
    ///  - `exited`=`<int>` containers with exit code of `<int>`
    ///  - `health`=(`starting`|`healthy`|`unhealthy`|`none`)
    ///  - `id`=`<ID>` a container's ID
    ///  - `isolation`=(`default`|`process`|`hyperv`) (Windows daemon only)
    ///  - `is-task`=`(true`|`false`)
    ///  - `label`=`key` or `label`=`"key=value"` of a container label
    ///  - `name`=`<name>` a container's name
    ///  - `network`=(`<network id>` or `<network name>`)
    ///  - `publish`=(`<port>[/<proto>]`|`<startport-endport>`/`[<proto>]`)
    ///  - `since`=(`<container id>` or `<container name>`)
    ///  - `status`=(`created`|`restarting`|`running`|`removing`|`paused`|`exited`|`dead`)
    ///  - `volume`=(`<volume name>` or `<mount point destination>`)
    #[serde(serialize_with = "crate::serialize_as_json")]
    pub filters: HashMap<T, Vec<T>>,
}

/// Options for the `logs` method.
/// This struct corresponds to the param options of the `GET /containers/(id)/logs` endpoint.
/// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerLogs) for more information.
#[derive(Serialize)]
pub struct LogsContainerOptions {
    pub follow: bool,
    pub stdout: bool,
    pub stderr: bool,
    pub since: i32,
    pub until: i32,
    pub timestamps: bool,
    pub tail: String,
}

impl Default for LogsContainerOptions {
    fn default() -> Self {
        LogsContainerOptions {
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
