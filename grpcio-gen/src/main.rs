// Copyright (c) 2023 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

use anyhow::Result;
use std::env::current_dir;
use std::path::PathBuf;
use clap::Parser;

/// Generates rust bindings for grpcio using prost
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Proto files to generate code from
    #[clap(value_names = ["PROTO"])]
    protos: Vec<String>,

    /// Include directories to use for dependencies
    #[clap(short = 'I', long, value_names = ["INCLUDE"])]
    includes: Vec<String>,

    /// Output directory to place the generated rust files
    #[clap(short, long)]
    out_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let out_dir = cli.out_dir.unwrap_or_else(|| current_dir().expect("Failed to get current directory."));

    grpcio_compiler::prost_codegen::compile_protos(
        &cli.protos,
        &cli.includes,
        out_dir.as_os_str().to_str().expect("Output directory has invalid UTF-8"),)?;
    Ok(())
}
