// Copyright (c) 2023 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

use std::path::PathBuf;
use clap::Parser;

/// Generates rust bindings for grpcio using prost
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Proto files to generate code from
    #[clap()]
    protos: Vec<String>,


    /// Output directory to place the generated rust files
    #[clap(short, long)]
    out_dir: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Hello {:?}!", cli.out_dir);
    println!("Hello {:?}!", cli.protos);
    Ok(())

}
