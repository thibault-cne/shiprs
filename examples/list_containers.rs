use std::collections::HashMap;

use shiprs::error::Result;
use shiprs::Docker;

// This example lists all exited containers available on the docker daemon.
fn main() -> Result<()> {
    let docker = Docker::new()?;
    let options = shiprs::container::ListOption {
        all: true,
        filters: HashMap::from([("status", vec!["exited"])]),
        ..Default::default()
    };
    let containers = docker.containers().list(Some(options))?;
    println!("{:?}", containers);

    Ok(())
}
