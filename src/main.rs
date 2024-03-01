use shiprs::{ContainerInfo, Docker, RequestBuilder};

fn main() {
    let docker = Docker::unix("/var/run/docker.sock").unwrap();
    let request = RequestBuilder::get("/containers/json").build();
    let response = docker.request::<Vec<ContainerInfo>>(request).unwrap();
    println!("{:?}", response);
}
