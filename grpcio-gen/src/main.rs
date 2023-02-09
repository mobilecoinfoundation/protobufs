// Copyright (c) 2023 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

use clap::{Parser, Subcommand};

/// Generates rust bindings for grpcio using prost
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Proto files to generate code from
    #[arg()]
    protos: Vec<String>,


    /// Output directory to place the generated rust files
    #[arg(short, long)]
    out_dir: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {


}
