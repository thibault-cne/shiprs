use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use shiprs_models::models::*;

use crate::http::request::RequestBuilder;
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
///
/// let container = docker
///     .containers()
///     .get("insert container id here")
///     .inspect(None)?;
///
/// println!("{:?}", container);
/// # Ok(())
/// # }
pub struct Container<'docker, T> {
    docker: &'docker Docker,
    id: T,
}

impl<'docker, T> Container<'docker, T>
where
    T: AsRef<str> + Eq + Hash + Serialize,
{
    pub(crate) fn new(docker: &'docker Docker, id: T) -> Self {
        Container { docker, id }
    }

    /// Inspects the docker container details.
    /// This corresponds to the `GET /containers/(id)/json` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerInspect) for more information.
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
    pub fn inspect(
        &self,
        options: Option<ContainerInspectOption>,
    ) -> Result<ContainerInspectResponse> {
        let url = format!("/containers/{}/json", self.id.as_ref());
        let request = RequestBuilder::<ContainerInspectOption, ()>::get(&*url)
            .query(options)
            .build();
        let response = self.docker.request(request)?;

        Ok(response.into_body().unwrap())
    }

    /// List processes running inside the container.
    /// This corresponds to the `GET /containers/(id)/top` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerTop) for more information.
    ///
    /// # Description
    /// On Unix systems, this is done by running the `ps` command.
    /// This endpoint is not supported on Windows.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let top = docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .top::<&str>(None)?;
    /// println!("{:?}", top);
    /// # Ok(())
    /// # }
    #[cfg(feature = "unix-socket")]
    pub fn top<O>(&self, options: Option<ContainerTopOption<O>>) -> Result<ContainerTopResponse>
    where
        O: Serialize + Into<String>,
    {
        let url = format!("/containers/{}/top", self.id.as_ref());
        let request = RequestBuilder::<ContainerTopOption<O>, ()>::get(&*url)
            .query(options)
            .build();
        let response = self.docker.request(request)?;

        Ok(response.into_body().unwrap())
    }

    /// Export a container
    /// This corresponds to the `GET /containers/(id)/export` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerExport) for more information.
    ///
    /// # Description
    /// Export the contents of a container as a tarball.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .export()?;
    /// # Ok(())
    /// # }
    pub fn export(&self) -> Result<()> {
        let url = format!("/containers/{}/export", self.id.as_ref());
        let request = RequestBuilder::<(), ()>::get(&*url).build();
        let _ = self.docker.request::<(), ()>(request)?;

        Ok(())
    }

    /// Get changes on a container’s filesystem
    /// This corresponds to the `GET /containers/(id)/changes` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerChanges) for more information.
    ///
    /// # Description
    /// Returns which files in a container's filesystem have been added, deleted, or modified.
    /// The Kind of modification can be one of:
    /// - 0: Modified ("C")
    /// - 1: Added ("A")
    /// - 2: Deleted ("D")
    ///
    /// let changes = docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .changes()?;
    ///
    /// for change in changes {
    ///   println!("{:?}", change);
    /// }
    /// # Ok(())
    /// # }
    pub fn changes(&self) -> Result<Vec<FilesystemChange>> {
        let url = format!("/containers/{}/changes", self.id.as_ref());
        let request = RequestBuilder::<(), ()>::get(&*url).build();
        let response = self.docker.request(request)?;

        Ok(response.into_body().unwrap())
    }

    /// Remove a container.
    /// This corresponds to the `DELETE /containers/(id)` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerDelete) for more information.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .remove(None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove(&self, options: Option<RemoveOption>) -> Result<()> {
        let url = format!("/containers/{}", self.id.as_ref());
        let request = RequestBuilder::<RemoveOption, ()>::delete(&*url)
            .query(options)
            .build();
        let _ = self.docker.request::<(), ()>(request)?;

        Ok(())
    }

    /// Resize a container TTY
    /// This corresponds to the `POST /containers/(id)/resize` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerResize) for more information.
    ///
    /// # Description
    /// Resize the TTY for a container.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    /// use shiprs::container::ResizeOption;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let options = ResizeOption {
    ///     h: 100,
    ///     w: 100,
    /// };
    ///
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .resize(Some(options))?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn resize(&self, options: Option<ResizeOption>) -> Result<()> {
        let url = format!("/containers/{}/resize", self.id.as_ref());
        let request = RequestBuilder::<ResizeOption, ()>::post(&*url)
            .query(options)
            .build();
        let _ = self.docker.request::<(), ()>(request)?;

        Ok(())
    }

    /// Start a container.
    /// This corresponds to the `POST /containers/(id)/start` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerStart) for more information.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    ///
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .start(None)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn start(&self, options: Option<StartOption>) -> Result<()> {
        let url = format!("/containers/{}/start", self.id.as_ref());
        let request = RequestBuilder::<StartOption, ()>::post(&*url)
            .query(options)
            .build();
        let _ = self.docker.request::<(), ()>(request)?;

        Ok(())
    }

    /// Stop a container.
    /// This corresponds to the `POST /containers/(id)/stop` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerStop) for more information.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    /// use shiprs::container::StopOption;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let options = StopOption {
    ///     t: Some(10),
    ///     ..Default::default()
    /// };
    ///
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .stop(Some(options))?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn stop(&self, options: Option<StopOption>) -> Result<()> {
        let url = format!("/containers/{}/stop", self.id.as_ref());
        let request = RequestBuilder::<StopOption, ()>::post(&*url)
            .query(options)
            .build();
        let _ = self.docker.request::<(), ()>(request)?;

        Ok(())
    }

    /// Restart a container.
    /// This corresponds to the `POST /containers/(id)/restart` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerRestart) for more information.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    /// use shiprs::container::RestartOption;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let options = RestartOption {
    ///     t: Some(10),
    ///     ..Default::default()
    /// };
    ///
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .restart(Some(options))?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn restart(&self, options: Option<RestartOption>) -> Result<()> {
        let url = format!("/containers/{}/restart", self.id.as_ref());
        let request = RequestBuilder::<RestartOption, ()>::post(&*url)
            .query(options)
            .build();
        let _ = self.docker.request::<(), ()>(request)?;

        Ok(())
    }

    /// Kill a container.
    /// This corresponds to the `POST /containers/(id)/kill` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerKill) for more information.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    ///
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .kill(None)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn kill(&self, options: Option<KillOption>) -> Result<()> {
        let url = format!("/containers/{}/kill", self.id.as_ref());
        let request = RequestBuilder::<KillOption, ()>::post(&*url)
            .query(options)
            .build();
        let _ = self.docker.request::<(), ()>(request)?;

        Ok(())
    }

    /// Update a container
    /// This corresponds to the `POST /containers/(id)/update` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerUpdate) for more information.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    /// use shiprs::container::UpdateConfig;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let config = UpdateConfig {
    ///     cpuset_cpus: Some("0,1"),
    ///     ..Default::default()
    /// };
    ///
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .update(config)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn update<C>(&self, config: UpdateConfig<C>) -> Result<()>
    where
        C: Into<String> + Eq + Hash + Serialize,
    {
        let url = format!("/containers/{}/update", self.id.as_ref());
        let request = RequestBuilder::<(), UpdateConfig<C>>::post(&*url)
            .body(config)
            .build();
        let _ = self.docker.request::<UpdateConfig<C>, ()>(request)?;

        Ok(())
    }

    /// Rename a container.
    /// This corresponds to the `POST /containers/(id)/rename` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerRename) for more information.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    /// use shiprs::container::RenameOption;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new().unwrap();
    /// let options = RenameOption {
    ///     name: "new_name",
    /// };
    ///
    /// docker
    ///     .containers()
    ///     .get("insert container id here")
    ///     .rename(options)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn rename<O>(&self, option: RenameOption<O>) -> Result<()>
    where
        O: AsRef<str> + Serialize,
    {
        let url = format!("/containers/{}/rename", self.id.as_ref());
        let request = RequestBuilder::<RenameOption<O>, ()>::post(&*url)
            .query(Some(option))
            .build();
        let _ = self.docker.request::<(), ()>(request)?;

        Ok(())
    }
}

/// Interface for interacting with docker containers.
///
/// # Example
/// ```no_run
/// # use shiprs::error::Result;
/// use shiprs::Docker;
///
/// # fn main() -> Result<()> {
/// let docker = Docker::new()?;
///
/// let containers = docker.containers().list::<&str>(None)?;
///
/// for container in containers {
///    println!("{:?}", container);
/// }
/// # Ok(())
/// # }
/// ```
pub struct Containers<'docker> {
    docker: &'docker Docker,
}

impl<'docker> Containers<'docker> {
    pub(crate) fn new(docker: &'docker Docker) -> Self {
        Containers { docker }
    }

    /// Create a new container.
    /// This corresponds to the `POST /containers/create` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerCreate) for more information.
    ///
    /// # Warning
    /// In order for the container to be created successfully, the image must be pulled before calling the create method.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    /// use shiprs::container::{CreateOption, CreateConfig};
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new()?;
    ///
    /// let options = CreateOption {
    ///     name: "my_container",
    ///     ..Default::default()
    /// };
    /// let config = CreateConfig {
    ///     image: Some("hello-world"),
    ///     cmd: Some(vec!["/hello"]),
    ///     ..Default::default()
    /// };
    ///
    /// let container = docker.containers().create(Some(options), config)?;
    /// println!("{:?}", container);
    /// # Ok(())
    /// # }
    /// ```
    pub fn create<O, C>(
        &self,
        options: Option<CreateOption<O>>,
        config: CreateConfig<C>,
    ) -> Result<ContainerCreateResponse>
    where
        O: Into<String> + Serialize,
        C: Into<String> + Eq + Hash + Serialize,
    {
        let url = "/containers/create";
        let request = RequestBuilder::<CreateOption<O>, CreateConfig<C>>::post(url)
            .query(options)
            .body(config)
            .build();
        let response = self.docker.request(request)?;

        Ok(response.into_body().unwrap())
    }

    /// Lists the docker containers.
    /// This corresponds to the `GET /containers/json` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerList) for more information.
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
    pub fn list<T>(&self, options: Option<ListOption<T>>) -> Result<Vec<ContainerSummary>>
    where
        T: Into<String> + std::hash::Hash + Eq + Serialize,
    {
        let url = "/containers/json";
        let request = RequestBuilder::<ListOption<T>, ()>::get(url)
            .query(options)
            .build();
        let response = self.docker.request(request)?;

        Ok(response.into_body().unwrap())
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
    pub fn get<T>(&self, id: T) -> Container<T>
    where
        T: AsRef<str> + Eq + Hash + Serialize,
    {
        Container::new(self.docker, id)
    }
}

/// Options for the `inspect` method.
/// This struct corresponds to the param options of the `GET /containers/(id)/json` endpoint.
/// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerInspect) for more information.
#[derive(Default, Serialize)]
pub struct ContainerInspectOption {
    /// Return the size of container as fields `SizeRw` and `SizeRootFs`.
    pub size: bool,
}

impl From<bool> for ContainerInspectOption {
    fn from(size: bool) -> Self {
        ContainerInspectOption { size }
    }
}

/// Parameters used for the [List Container API](Containers::list)
///
/// ## Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use shiprs::container::ListOption;
///
/// // Get all running containers
/// let options = ListOption {
///     all: true,
///     filters: HashMap::from([("status", vec!["running"])]),
///     ..Default::default()
/// };
/// ```
///
/// ```rust
/// use shiprs::container::ListOption;
///
/// // Get all containers
/// let options: ListOption<&str> = ListOption {
///    all: true,
///   ..Default::default()
/// };
/// ```
#[derive(Default, Serialize)]
pub struct ListOption<T>
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

/// Options for the `create` method.
/// This struct corresponds to the param options of the `POST /containers/create` endpoint.
/// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerCreate) for more information.
#[derive(Default, Serialize)]
pub struct CreateOption<T>
where
    T: Into<String> + Serialize,
{
    /// Assign the specified name to the container. Must match `/?[a-zA-Z0-9][a-zA-Z0-9_.-]+`.
    pub name: T,

    /// Platform in the format os[/arch[/variant]] used for image lookup.
    /// When specified, the daemon checks if the requested image is present in
    /// the local image cache with the given OS and Architecture, and otherwise returns a 404 status.
    ///
    /// If the option is not set, the host's native OS and Architecture are used to look up the image in the image cache.
    /// However, if no platform is passed and the given image does exist in the local image cache,
    /// but its OS or architecture does not match, the container is created with the available image,
    /// and a warning is added to the Warnings field in the response, for example :
    ///
    /// ```plaintext
    /// WARNING: The requested image's platform (linux/arm64/v8) does not
    ///     match the detected host platform (linux/amd64) and no
    ///     specific platform was requested
    /// ```
    pub platform: Option<T>,
}

/// This container's networking configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(missing_docs)]
pub struct NetworkingConfig<T: Into<String> + Hash + Eq> {
    pub endpoints_config: HashMap<T, EndpointSettings>,
}

/// Container to create.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateConfig<T>
where
    T: Into<String> + Eq + Hash,
{
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<T>,

    /// The domain name to use for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<T>,

    /// The user that commands are run as inside the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<T>,

    /// Whether to attach to `stdin`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,

    /// Whether to attach to `stdout`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,

    /// Whether to attach to `stderr`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,

    /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<HashMap<T, HashMap<(), ()>>>,

    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,

    /// Open `stdin`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,

    /// Close `stdin` after one attached client disconnects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,

    /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<T>>,

    /// Command to run specified as a string or an array of strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<T>>,

    /// A TEST to perform TO Check that the container is healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthConfig>,

    /// Command is already escaped (Windows only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,

    /// The name of the image to use when creating the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<T>,

    /// An object mapping mount point paths inside the container to empty objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<T, HashMap<(), ()>>>,

    /// The working directory for commands to run in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<T>,

    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<T>>,

    /// Disable networking for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,

    /// MAC address of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<T>,

    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_build: Option<Vec<T>>,

    /// User-defined key/value metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<T, T>>,

    /// Signal to stop a container as a string or unsigned integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<T>,

    /// Timeout to stop a container in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,

    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<Vec<T>>,

    /// Container configuration that depends on the host we are running on.
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_config: Option<HostConfig>,

    /// This container's networking configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking_config: Option<NetworkingConfig<T>>,
}

impl From<ContainerConfig> for CreateConfig<String> {
    fn from(value: ContainerConfig) -> Self {
        CreateConfig {
            hostname: value.hostname,
            domainname: value.domainname,
            user: value.user,
            attach_stdin: value.attach_stdin,
            attach_stdout: value.attach_stdout,
            attach_stderr: value.attach_stderr,
            exposed_ports: value.exposed_ports,
            tty: value.tty,
            open_stdin: value.open_stdin,
            stdin_once: value.stdin_once,
            env: value.env,
            cmd: value.cmd,
            healthcheck: value.healthcheck,
            args_escaped: value.args_escaped,
            image: value.image,
            volumes: value.volumes,
            working_dir: value.working_dir,
            entrypoint: value.entrypoint,
            network_disabled: value.network_disabled,
            mac_address: value.mac_address,
            on_build: value.on_build,
            labels: value.labels,
            stop_signal: value.stop_signal,
            stop_timeout: value.stop_timeout,
            shell: value.shell,
            host_config: None,
            networking_config: None,
        }
    }
}

#[derive(Default, Serialize)]
pub struct ContainerTopOption<T>
where
    T: Serialize + Into<String>,
{
    pub ps_args: Option<T>,
}

#[derive(Default, Serialize)]
pub struct RemoveOption {
    /// If the container is running, kill it before removing it.
    pub force: bool,

    /// Remove the volumes associated with the container.
    pub v: bool,

    /// Remove the specified link associated with the container.
    pub link: bool,
}

#[derive(Default, Serialize)]
pub struct ResizeOption {
    /// Height of the TTY session in characters
    pub h: u32,

    /// Width of the TTY session in characters
    pub w: u32,
}

#[derive(Default, Serialize)]
pub struct StartOption {
    /// Override the key sequence for detaching a container.
    /// Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detachKeys")]
    pub detach_keys: Option<String>,
}

#[derive(Default, Serialize)]
pub struct StopOption {
    /// Signal to send to the container as an integer or string (e.g. `SIGINT`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<String>,

    /// Number of seconds to wait before killing the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<u32>,
}

#[derive(Default, Serialize)]
pub struct RestartOption {
    /// Signal to send to the container as an integer or string (e.g. `SIGINT`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<String>,

    /// Number of seconds to wait before killing the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<u32>,
}

#[derive(Default, Serialize)]
pub struct KillOption {
    /// Signal to send to the container as an integer or string (e.g. `SIGINT`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<String>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateConfig<T>
where
    T: Into<String> + Eq + Hash,
{
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i64>,

    /// Memory limit in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,

    /// Path to `cgroups` under which the container's `cgroup` is created.
    /// If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process.
    /// Cgroups are created if they do not already exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<T>,

    /// Block IO weight (relative weight) accepts a weight value between 0 and 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<u16>,

    /// Block IO weight (relative device weight).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<ResourcesBlkioWeightDevice>>,

    /// Limit read rate (bytes per second) from a device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (bytes per second) to a device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,

    /// Limit read rate (IO per second) from a device.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (IO per second) to a device.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Option<Vec<ThrottleDevice>>,

    /// The length of a CPU period in microseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,

    /// Microseconds of CPU time that the container can get in a CPU period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,

    /// The length of a CPU real-time period in microseconds.
    /// Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,

    /// The length of a CPU real-time runtime in microseconds.
    /// Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,

    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<T>,

    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1).
    /// Only effective on NUMA systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<T>,

    /// A list of devices to add to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceMapping>>,

    /// A list of cgroup rules to apply to the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<T>>,

    /// A list of requests for devices to be sent to device drivers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<DeviceRequest>>,

    /// Hard limit for kernel TCP buffer memory (in bytes).
    /// Depending on the OCI runtime in use, this option may be ignored.
    /// It is no longer supported by the default (runc) runtime.
    ///
    /// This field is omitted when empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,

    /// Memory soft limit in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,

    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,

    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<u8>,

    /// CPU quota in units of 10-9 CPUs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,

    /// Disable OOM Killer for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,

    /// Run an init inside the container that forwards signals and reaps processes.
    /// This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,

    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or null to not change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,

    /// A list of resource limits to set in the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<ResourcesUlimits>>,

    /// The number of usable CPUs (Windows only).
    ///
    /// On Windows Server containers, the processor resource controls are mutually exclusive.
    /// The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[cfg(feature = "windows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,

    /// The usable percentage of the available CPUs (Windows only).
    ///
    /// On Windows Server containers, the processor resource controls are mutually exclusive.
    /// The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[cfg(feature = "windows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,

    /// Maximum IOps for the container system drive (Windows only)
    #[cfg(feature = "windows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_iops: Option<i64>,

    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[cfg(feature = "windows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<i64>,

    /// The behavior to apply when the container exits. The default is not to restart.
    ///
    /// An ever increasing delay (double the previous delay, starting at 100ms) is added before each restart to prevent flooding the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<RestartPolicy>,
}

#[derive(Default, Serialize)]
pub struct RenameOption<T>
where
    T: AsRef<str> + Serialize,
{
    /// The new name for the container.
    pub name: T,
}
