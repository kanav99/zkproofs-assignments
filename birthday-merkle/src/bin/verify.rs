
use std::{error::Error, fs, path::PathBuf};

use clap::Parser;
use risc0_zkvm::Receipt;
use birthday_core::{
    birthday::{BirthdayMerkleTree,SimpleDate}, merkle::Node, Journal
};
use birthday_methods::BIRTHDAY_ID;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// Input file path to fetch the receipt. 
    #[clap(short = 'r', long, value_parser, default_value = "./receipt.bin", value_hint = clap::ValueHint::FilePath)]
    receipt: PathBuf,
    /// Input file path that contains the root of the Merkle tree.
    #[clap(short = 'm', long, value_parser, default_value = "./merkle_root.bin", value_hint = clap::ValueHint::FilePath)]
    m_r: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    // Load and verify the receipt file.
    let receipt: Receipt = bincode::deserialize(&fs::read(&args.receipt)?)?;
    receipt.verify(BIRTHDAY_ID)?;
    // TODO: Initialize the Merkle root received from the file
    let root :Node = bincode::deserialize(&fs::read(&args.m_r)?)?;

    // Check consistency of the journal against the input Where's Waldo image.
    let journal: Journal = receipt.journal.decode()?;
//************************************YOUR CODE STARTS HERE************************************

// TODO: Check the consistency of the Merkle roots
    assert_eq!(journal.bd_merkle_tree, root);

//*************************************YOUR CODE ENDS HERE*************************************
  
    Ok(())
}