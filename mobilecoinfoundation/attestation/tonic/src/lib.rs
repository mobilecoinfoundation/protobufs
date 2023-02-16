// Copyright (c) 2023 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

use mc_attestation_messages::*;

include!(concat!(
    env!("OUT_DIR"),
    "/mobilecoinfoundation.attestation.v1.tonic.rs"
));
