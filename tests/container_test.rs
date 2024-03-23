use shiprs::{
    container::{ListOption, RenameOption},
    error::Result,
    Docker,
};

mod common;
use common::*;

#[test]
fn integration_test_list_containers() -> Result<()> {
    let docker = Docker::new()?;

    let image = format!("{}hello-world:linux", registry_http_addr());
    create_container(&docker, &image, "integration_test_list_containers")?;

    let options = ListOption::<String> {
        all: true,
        ..Default::default()
    };

    let containers = docker.containers().list(Some(options)).unwrap();

    assert_ne!(containers.len(), 0);
    assert!(containers
        .iter()
        .any(|c| c.image.as_ref().unwrap() == &image));

    remove_container(&docker, "integration_test_list_containers")?;

    Ok(())
}

#[test]
fn integration_test_inspect_container() -> Result<()> {
    let docker = Docker::new()?;

    let image = format!("{}hello-world:linux", registry_http_addr());
    create_container(&docker, &image, "integration_test_inspect_container")?;

    let container = docker
        .containers()
        .get("integration_test_inspect_container")
        .inspect(None)?;

    assert_eq!(container.config.unwrap().image.unwrap(), image);
    assert_eq!(
        container.name.unwrap(),
        "/integration_test_inspect_container"
    );

    remove_container(&docker, "integration_test_inspect_container")?;

    Ok(())
}

#[test]
fn integration_test_start_container() -> Result<()> {
    let docker = Docker::new()?;

    create_daemon(&docker, "integration_test_start_container")?;

    let container = docker
        .containers()
        .get("integration_test_start_container")
        .inspect(None)?;

    assert!(container.state.unwrap().running.unwrap());

    remove_daemon(&docker, "integration_test_start_container")?;

    Ok(())
}

#[test]
fn integration_test_stop_container() -> Result<()> {
    let docker = Docker::new()?;

    create_daemon(&docker, "integration_test_stop_container")?;

    docker
        .containers()
        .get("integration_test_stop_container")
        .stop(None)?;

    let container = docker
        .containers()
        .get("integration_test_stop_container")
        .inspect(None)?;

    assert!(!container.state.unwrap().running.unwrap());

    remove_container(&docker, "integration_test_stop_container")?;

    Ok(())
}

#[test]
fn integration_test_restart_container() -> Result<()> {
    let docker = Docker::new()?;

    create_daemon(&docker, "integration_test_restart_container")?;

    let container = docker
        .containers()
        .get("integration_test_restart_container")
        .inspect(None)?;

    let start_time = container.state.unwrap().started_at.unwrap();

    docker
        .containers()
        .get("integration_test_restart_container")
        .restart(None)?;

    let container = docker
        .containers()
        .get("integration_test_restart_container")
        .inspect(None)?;

    assert!(container.state.as_ref().unwrap().running.unwrap());
    assert_ne!(container.state.unwrap().started_at.unwrap(), start_time);

    remove_daemon(&docker, "integration_test_restart_container")?;

    Ok(())
}

#[test]
fn integration_test_rename_container() -> Result<()> {
    let docker = Docker::new()?;

    let image = format!("{}hello-world:linux", registry_http_addr());
    let container_id = create_container(&docker, &image, "integration_test_rename_container")?.id;

    let option = RenameOption {
        name: "integration_test_rename_container_2",
    };

    let container = docker.containers().get(&container_id);
    container.rename(option)?;
    let inspect = container.inspect(None)?;

    assert_eq!(
        inspect.name.unwrap(),
        "/integration_test_rename_container_2"
    );

    remove_container(&docker, &container_id)?;

    Ok(())
}
