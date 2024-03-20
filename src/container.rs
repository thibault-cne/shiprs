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
    pub fn inspect(
        &self,
        options: Option<ContainerInspectOption>,
    ) -> Result<ContainerInspectResponse> {
        let url = format!("/containers/{}/json", self.id);
        let request = RequestBuilder::<ContainerInspectOption, ()>::get(&*url)
            .query(options)
            .build();
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
    pub fn logs(&self, options: Option<ContainerLogsOption>) -> Result<String> {
        let url = format!("/containers/{}/logs", self.id);
        let request = RequestBuilder::<ContainerLogsOption, ()>::get(&*url)
            .query(options)
            .build();
        let response = self.docker.request(request)?;

        Ok(response.into_body())
    }

    /// List processes running inside the container.
    /// This corresponds to the `GET /containers/(id)/top` endpoint.
    /// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerTop) for more information.
    ///
    /// # Description
    /// On Unix systems, this is done by running the `ps` command.
    /// This endpoint is not supported on Windows.
    ///
    /// # Parameters
    /// - `options`: ContainerTopOption, used to provide options to the top method.
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
    pub fn top<T>(&self, options: Option<ContainerTopOption<T>>) -> Result<ContainerTopResponse>
    where
        T: Serialize + Into<String>,
    {
        let url = format!("/containers/{}/top", self.id);
        let request = RequestBuilder::<ContainerTopOption<T>, ()>::get(&*url)
            .query(options)
            .build();
        let response = self.docker.request(request)?;

        Ok(response.into_body())
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
        let url = format!("/containers/{}/export", self.id);
        let request = RequestBuilder::<(), ()>::get(&*url).build();
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
/// let containers = docker.containers().list::<&str>(None)?;
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
    /// # Parameters
    /// - `options`: ContainerCreateOption, used to provide options to the create method.
    /// - `config`: Config, used to provide the configuration for the newly created container.
    ///
    /// # Warning
    /// In order for the container to be created successfully, the image must be pulled before calling the create method.
    ///
    /// # Example
    /// ```no_run
    /// # use shiprs::error::Result;
    /// use shiprs::Docker;
    ///
    /// # fn main() -> Result<()> {
    /// let docker = Docker::new()?;
    ///
    /// let options = shiprs::container::ContainerCreateOption {
    ///     name: "my_container",
    ///     ..Default::default()
    /// };
    /// let config = shiprs::container::Config {
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
        options: Option<ContainerCreateOption<O>>,
        config: Config<C>,
    ) -> Result<ContainerCreateResponse>
    where
        O: Into<String> + Serialize,
        C: Into<String> + Eq + Hash + Serialize,
    {
        let url = "/containers/create";
        let request = RequestBuilder::<ContainerCreateOption<O>, Config<C>>::post(url)
            .query(options)
            .body(config)
            .build();
        let response = self.docker.request(request)?;

        Ok(response.into_body())
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
    pub fn list<T>(&self, options: Option<ContainerListOption<T>>) -> Result<Vec<ContainerSummary>>
    where
        T: Into<String> + std::hash::Hash + Eq + Serialize,
    {
        let url = "/containers/json";
        let request = RequestBuilder::<ContainerListOption<T>, ()>::get(url)
            .query(options)
            .build();
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
///
/// use shiprs::container::ContainerListOption;
///
/// // Get all running containers
/// let options = ContainerListOption {
///     all: true,
///     filters: HashMap::from([("status", vec!["running"])]),
///     ..Default::default()
/// };
/// ```
///
/// ```rust
/// use shiprs::container::ContainerListOption;
///
/// // Get all containers
/// let options: ContainerListOption<&str> = ContainerListOption {
///    all: true,
///   ..Default::default()
/// };
/// ```
#[derive(Default, Serialize)]
pub struct ContainerListOption<T>
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
pub struct ContainerLogsOption {
    /// Keep connection after returning logs.
    pub follow: bool,
    /// Return logs from `stdout`.
    pub stdout: bool,
    /// Return logs from `stderr`.
    pub stderr: bool,
    /// Only return logs since this time, as a UNIX timestamp.
    pub since: i32,
    /// Only return logs before this time, as a UNIX timestamp.
    pub until: i32,
    /// Add timestamps to every log line.
    pub timestamps: bool,
    /// Only return this number of log lines from the end of the logs.
    /// Specify as an integer or `all` to output all logs.
    pub tail: String,
}

impl Default for ContainerLogsOption {
    fn default() -> Self {
        ContainerLogsOption {
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

/// Options for the `create` method.
/// This struct corresponds to the param options of the `POST /containers/create` endpoint.
/// See the [API documentation](https://docs.docker.com/engine/api/v1.44/#tag/Container/operation/ContainerCreate) for more information.
#[derive(Default, Serialize)]
pub struct ContainerCreateOption<T>
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
pub struct Config<T>
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

impl From<ContainerConfig> for Config<String> {
    fn from(value: ContainerConfig) -> Self {
        Config {
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
