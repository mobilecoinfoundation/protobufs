// Copyright (c) 2023 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]

// the grpcio code generator assumes the messages and the services are in the
// same module.
use mc_attestation_messages::*;

include!(concat!(
    env!("OUT_DIR"),
    "/mobilecoinfoundation.attestation.v1.rs"
));
