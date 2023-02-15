// Copyright (c) 2023 The MobileCoin Foundation

//! Generate the grpcio code for the gRPC service.

use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    grpcio_compiler::prost_codegen::compile_protos(
        &["../v1/services.proto"],
        &["../../../", "../"],
        &out_dir,
    )?;

    // This is not ideal and is likely to be fragile longer term.
    // Ideally there would be a `protoc` plugin like
    // https://crates.io/crates/protoc-gen-tonic for generating just the
    // grpcio services.
    let out_dir = PathBuf::from(out_dir).join("mobilecoinfoundation.attestation.v1.rs");
    remove_messages_from_grpcio_services(out_dir)?;
    Ok(())
}

/// Removes the message types from the generated grpcio rust file.
///
/// # Arguments:
/// * `path` - The path to the generated grpcio file.
fn remove_messages_from_grpcio_services(
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path)?;

    // The services are appended after the message types. Each service is named
    // `const METHOD_...`. This finds the first occurrence of that string.
    let start = contents
        .find("const METHOD")
        .expect("Expecting the services to be present");
    let (_, services) = contents.split_at(start);
    fs::write(path, services)?;
    Ok(())
}
