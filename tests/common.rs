use std::process::Command;

use shiprs::error::Error;

pub fn create_container(image_name: &str, container_name: &str) -> Result<(), Error> {
    let _ = Command::new("docker")
        .args(["create", "--name", container_name, image_name])
        .output()?;

    Ok(())
}

pub fn remove_container(container_name: &str) -> Result<(), Error> {
    let _ = Command::new("docker")
        .args(["rm", "-f", container_name])
        .output()?;

    Ok(())
}
