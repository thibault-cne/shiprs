#![allow(missing_docs)]
use std::{path::Path, process::Command};

fn main() {
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Check for modification for rerun
    let build_rs = base_path.join("build.rs");
    println!("cargo:rerun-if-changed={}", build_rs.to_str().unwrap());

    ensure_mvn();
    build_models();
    cleanup(base_path);
}

fn ensure_mvn() {
    if Command::new("mvn").arg("--version").status().is_err() {
        panic!("Maven is not installed");
    }
}

fn build_models() {
    Command::new("mvn")
        .args(["clean", "compiler:compile", "generate-resources"])
        .output()
        .expect("Failed to build models");
}

fn cleanup<P: AsRef<Path>>(base_path: P) {
    let dirs = [
        "target",
        "src/client",
        "src/server",
        "api",
        "examples",
        ".swagger-codegen",
    ];

    let files = [".swagger-codegen-ignore"];

    for dir in dirs.iter() {
        let path = base_path.as_ref().join(dir);
        if path.exists() {
            std::fs::remove_dir_all(path).expect("Failed to remove directory");
        }
    }

    for file in files.iter() {
        let path = base_path.as_ref().join(file);
        if path.exists() {
            std::fs::remove_file(path).expect("Failed to remove file");
        }
    }
}
