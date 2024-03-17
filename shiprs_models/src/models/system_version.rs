/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.44) is used. For example, calling `/info` is the same as calling `/v1.44/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a [base64url encoded](https://tools.ietf.org/html/rfc4648#section-5) (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ``` 
 *
 * The version of the OpenAPI document: 1.44
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// SystemVersion : Response of Engine API: GET \"/version\" 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemVersion {
    #[serde(rename = "Platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<models::SystemVersionPlatform>>,
    /// Information about system components 
    #[serde(rename = "Components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<models::SystemVersionComponentsInner>>,
    /// The version of the daemon
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The default (and highest) API version that is supported by the daemon 
    #[serde(rename = "ApiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The minimum API version that is supported by the daemon 
    #[serde(rename = "MinAPIVersion", skip_serializing_if = "Option::is_none")]
    pub min_api_version: Option<String>,
    /// The Git commit of the source code that was used to build the daemon 
    #[serde(rename = "GitCommit", skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    /// The version Go used to compile the daemon, and the version of the Go runtime in use. 
    #[serde(rename = "GoVersion", skip_serializing_if = "Option::is_none")]
    pub go_version: Option<String>,
    /// The operating system that the daemon is running on (\"linux\" or \"windows\") 
    #[serde(rename = "Os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The architecture that the daemon is running on 
    #[serde(rename = "Arch", skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// The kernel version (`uname -r`) that the daemon is running on.  This field is omitted when empty. 
    #[serde(rename = "KernelVersion", skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// Indicates if the daemon is started with experimental features enabled.  This field is omitted when empty / false. 
    #[serde(rename = "Experimental", skip_serializing_if = "Option::is_none")]
    pub experimental: Option<bool>,
    /// The date and time that the daemon was compiled. 
    #[serde(rename = "BuildTime", skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
}

impl SystemVersion {
    /// Response of Engine API: GET \"/version\" 
    pub fn new() -> SystemVersion {
        SystemVersion {
            platform: None,
            components: None,
            version: None,
            api_version: None,
            min_api_version: None,
            git_commit: None,
            go_version: None,
            os: None,
            arch: None,
            kernel_version: None,
            experimental: None,
            build_time: None,
        }
    }
}

