use anyhow::Result;
use std::{fs, path::Path};

const PATH: &str = "src/pb";

fn main() -> Result<()> {
    if !Path::new(PATH).exists() {
        fs::create_dir_all(PATH)?;
    }
    prost_build::Config::new()
        .out_dir(PATH)
        .compile_protos(&["../../protos/messages.proto"], &["../../protos"])?;

    Ok(())
}
