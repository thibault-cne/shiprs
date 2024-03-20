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
