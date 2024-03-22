use shiprs::container::{Config, ContainerCreateOption};
use shiprs::error::Error;
use shiprs::Docker;

pub fn create_container(
    docker: &Docker,
    image_name: &str,
    container_name: &str,
) -> Result<(), Error> {
    let option = ContainerCreateOption {
        name: container_name.to_string(),
        ..Default::default()
    };
    let config = Config {
        image: Some(image_name),
        ..Default::default()
    };

    let _ = docker.containers().create(Some(option), config)?;

    Ok(())
}

pub fn remove_container(docker: &Docker, container_name: &str) -> Result<(), Error> {
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
