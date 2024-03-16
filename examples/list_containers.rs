use std::collections::HashMap;

use shiprs::container::ListContainersOption;
use shiprs::Docker;

// This example lists all exited containers available on the docker daemon.
fn main() {
    let docker = Docker::new().unwrap();
    let options = ListContainersOption {
        all: true,
        filters: HashMap::from([("status", vec!["exited"])]),
        ..Default::default()
    };
    let containers = docker.containers().list(Some(options));
    println!("{:?}", containers);
}
