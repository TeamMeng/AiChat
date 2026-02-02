use anyhow::Result;
use std::{fs, path::Path, process::Command};

const PATH: &str = "src/pb";

fn main() -> Result<()> {
    if !Path::new(PATH).exists() {
        fs::create_dir_all(PATH)?;
    }
    prost_build::Config::new()
        .out_dir(PATH)
        .compile_protos(&["../../protos/messages.proto"], &["../../protos"])?;

    let status = Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("failed to format code");
    assert!(status.success());

    Ok(())
}
