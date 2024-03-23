use shiprs::error::Result;
use shiprs::Docker;

// Note that in order for the following code to work, you need to have the `hello-world` image.
fn main() -> Result<()> {
    let docker = Docker::new()?;

    let options = shiprs::container::CreateOption {
        name: "my_container",
        ..Default::default()
    };
    let config = shiprs::container::CreateConfig {
        image: Some("hello-world"),
        cmd: Some(vec!["/hello"]),
        ..Default::default()
    };

    let container = docker.containers().create(Some(options), config)?;
    println!("{:?}", container);

    Ok(())
}
