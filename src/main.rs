use shiprs::Docker;

fn main() {
    let docker = Docker::new().unwrap();
    let container = docker
        .containers()
        .get("c7ed178f5b9f63443a8fcd569f408d458bf6f177ab8a8d6a49e9598265a0a493")
        .inspect()
        .unwrap();
    println!("{:?}", container);

    let with_size = docker
        .containers()
        .get("c7ed178f5b9f63443a8fcd569f408d458bf6f177ab8a8d6a49e9598265a0a493")
        .inspect()
        .unwrap();
    println!("{:?}", with_size);
}
