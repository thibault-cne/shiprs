use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::path::Path;

use std::process::Command;

const LIB_RS_CONTENT: &str = r#"#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod models;"#;

fn main() -> Result<()> {
    ensure_openapi_generator_cli()?;
    generate_models()?;

    write_string_to_file("src/lib.rs", LIB_RS_CONTENT)?;

    // Cleanup generated files
    cleanup_generated_files()?;

    Ok(())
}

fn cleanup_generated_files() -> Result<()> {
    let generated_files = ["openapitools.json"];

    let generated_dirs = ["src/apis", ".openapi-generator"];

    for file in generated_files.iter() {
        if std::fs::metadata(file).is_ok() {
            std::fs::remove_file(file)?;
        }
    }

    for dir in generated_dirs.iter() {
        if std::fs::metadata(dir).is_ok() {
            std::fs::remove_dir_all(dir)?;
        }
    }

    Ok(())
}

fn ensure_openapi_generator_cli() -> Result<()> {
    let output = Command::new("openapi-generator-cli")
        .arg("version")
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "openapi-generator-cli not found in PATH",
        ));
    }

    Ok(())
}

fn generate_models() -> Result<()> {
    let mut cmd = Command::new("openapi-generator-cli");
    cmd.args([
        "generate",
        "-i",
        "https://docs.docker.com/reference/engine/v1.44.yaml",
        "-g",
        "rust",
        "-c",
        "openapi_generator_config.yaml",
    ]);

    let output = cmd.output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "openapi-generator-cli failed with status: {}",
                output.status
            ),
        ));
    }

    Ok(())
}

fn write_string_to_file<P: AsRef<Path>>(path: P, contents: impl AsRef<str>) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_ref().as_bytes())?;

    Ok(())
}
