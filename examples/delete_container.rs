use shiprs::error::Result;
use shiprs::Docker;

// Note that in order for the following code to work, you need to have the `hello-world` image.
fn main() -> Result<()> {
    let docker = Docker::new()?;

    let options = shiprs::container::ContainerCreateOption {
        name: "my_container",
        ..Default::default()
    };
    let config = shiprs::container::Config {
        image: Some("hello-world"),
        cmd: Some(vec!["/hello"]),
        ..Default::default()
    };

    let _ = docker.containers().create(Some(options), config)?;

    docker.containers().get("my_container").remove(None)?;

    Ok(())
}
