use shiprs::container::{CreateConfig, CreateOption};
use shiprs::error::Result;
use shiprs::{Docker, DockerResponse};

use shiprs_models::models::ContainerCreateResponse;

#[macro_use]
mod macros;

pub fn create_container(
    docker: &Docker,
    image_name: &str,
    container_name: &str,
) -> Result<DockerResponse<ContainerCreateResponse>> {
    let option = CreateOption {
        name: container_name.to_string(),
        ..Default::default()
    };
    let config = CreateConfig {
        image: Some(image_name),
        ..Default::default()
    };

    docker.containers().create(Some(option), config)
}

pub fn create_daemon(
    docker: &Docker,
    container_name: &str,
) -> Result<DockerResponse<ContainerCreateResponse>> {
    let image_name = format!("{}androw/uhttpd", registry_http_addr());

    let option = CreateOption {
        name: container_name.to_string(),
        ..Default::default()
    };
    let config = CreateConfig {
        image: Some(image_name),
        ..Default::default()
    };

    let result = docker.containers().create(Some(option), config)?;
    let result_ref = success_ref!(result).body()?;

    assert!(!result_ref.id.is_empty());

    docker.containers().get(container_name).start(None)?;

    Ok(result)
}

pub fn remove_daemon(docker: &Docker, container_name: &str) -> Result<DockerResponse<()>> {
    docker.containers().get(container_name).stop(None)?;
    remove_container(docker, container_name)
}

pub fn remove_container(docker: &Docker, container_name: &str) -> Result<DockerResponse<()>> {
    docker.containers().get(container_name).remove(None)
}

pub fn registry_http_addr() -> String {
    if std::env::var("DISABLE_REGISTRY").is_ok() {
        String::new()
    } else {
        format!(
            "{}/",
            std::env::var("REGISTRY_HTTP_ADDR").unwrap_or_else(|_| "localhost:5000".to_string())
        )
    }
}
