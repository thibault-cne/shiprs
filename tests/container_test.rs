use shiprs::{container::ContainerListOption, error::Error, Docker};

mod common;
use common::*;

#[test]
fn test_list_containers() -> Result<(), Error> {
    let docker = Docker::new()?;

    let image = format!("{}hello-world:linux", registry_http_addr());
    create_container(&docker, &image, "test_list_containers")?;

    let options = ContainerListOption::<String> {
        all: true,
        ..Default::default()
    };

    let containers = docker.containers().list(Some(options)).unwrap();

    assert_ne!(containers.len(), 0);
    assert!(containers
        .iter()
        .any(|c| c.image.as_ref().unwrap() == "hello-world"));

    Ok(())
}
