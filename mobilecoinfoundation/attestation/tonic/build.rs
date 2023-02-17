// Copyright (c) 2023 The MobileCoin Foundation

//! Generate the tonic code for the gRPC service.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = tonic_build::configure().extern_path(
        ".mobilecoinfoundation.attestation.v1",
        "::mc_attestation_messages",
    );
    builder.compile(&["../v1/services.proto"], &["../../../", "../"])?;

    Ok(())
}
