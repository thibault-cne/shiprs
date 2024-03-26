use shiprs::{error::Result, Docker};
use shiprs_models::models::ErrorResponse;

pub mod common;
use common::*;

#[test]
fn integration_test_error_parsing() -> Result<()> {
    let docker = Docker::new()?;
    let image = format!("{}unknown_image:unkown", registry_http_addr());

    let res = create_container(&docker, &image, "integration_test_error_parsing")?;

    assert!(res.is_error());

    let err_s = res.error().unwrap().body()?;
    let expected_err = ErrorResponse {
        message: "No such image: localhost:5000/unknown_image:unkown".to_string(),
    };

    assert_eq!(err_s, expected_err);

    Ok(())
}
