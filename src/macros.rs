macro_rules! docker_response {
    ($resp:expr) => {
        crate::docker::DockerResponse::from($resp)
    };
}
