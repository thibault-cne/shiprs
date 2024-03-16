use shiprs::Docker;

fn main() {
    let docker = Docker::new().unwrap();
    let container = docker
        .containers()
        .get("7dfb88711a3b8d3ee86f204dd615c349f14dd297e185312ac02c1186d569d8b4")
        .inspect(None);

    println!("{:?}", container);
}
