// Copyright (c) 2023 The MobileCoin Foundation

//! Generate the prost messages for the gRPC service.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["../v1/attest.proto"], &["../../../", "../"])?;
    Ok(())
}
