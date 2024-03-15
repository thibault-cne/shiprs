use shiprs::Docker;

// This example lists all containers available on the docker daemon.
fn main() {
    let docker = Docker::new().unwrap();
    let containers = docker.containers().list();
    println!("{:?}", containers);
}
