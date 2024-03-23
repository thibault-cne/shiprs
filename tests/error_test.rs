use shiprs::{error::Result, Docker};

pub mod common;
use common::*;

#[test]
fn integration_test_error_parsing() -> Result<()> {
    let docker = Docker::new()?;
    let image = format!("{}unknown_image:unkown", registry_http_addr());

    let res = create_container(&docker, &image, "integration_test_error_parsing");

    assert!(res.is_err());

    let err_s = res.unwrap_err().to_string();

    assert_eq!(
        err_s,
        "http response error: No such image: localhost:8080/androw/uhttpd:latest"
    );

    Ok(())
}
