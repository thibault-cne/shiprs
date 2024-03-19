use shiprs::error::Result;
use shiprs::Docker;

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

    let container = docker.containers().create(Some(options), config)?;
    println!("{:?}", container);

    Ok(())
}
