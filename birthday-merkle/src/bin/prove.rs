

use std::{error::Error,fs,path::PathBuf};
use clap::Parser;
use risc0_zkvm::{default_prover, ExecutorEnv};
use birthday_core::{
    birthday::{SimpleDate,BirthdayMerkleTree},
    merkle::SYS_VECTOR_ORACLE,
    PrivateInput,
};
use birthday_methods::BIRTHDAY_ELF;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Output file path to save the receipt.
    #[clap(short = 'r', long, value_parser, default_value = "./receipt.bin", value_hint = clap::ValueHint::FilePath)]
    receipt: PathBuf,
    /// Output file path to save the root of the Merkle tree.
    #[clap(short = 'm', long, value_parser, default_value = "./merkle_root.bin", value_hint = clap::ValueHint::FilePath)]
    m_r: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

//************************************YOUR CODE STARTS HERE************************************

// TODO: Populate a merkle tree that includes a specific date
    let dates: Vec<SimpleDate> = [
        SimpleDate::new(2000, 1, 1),
        SimpleDate::new(2001, 1, 1),
        SimpleDate::new(2002, 1, 1),
        SimpleDate::new(2003, 1, 1),
        SimpleDate::new(2004, 1, 1),
        SimpleDate::new(2005, 1, 1),
        SimpleDate::new(2006, 1, 1),
        SimpleDate::new(2007, 1, 1),
        SimpleDate::new(2008, 1, 1),
        SimpleDate::new(2009, 1, 1),
        SimpleDate::new(2010, 1, 1),
        SimpleDate::new(2011, 1, 1),
        SimpleDate::new(2012, 1, 1),
        SimpleDate::new(2013, 1, 1),
        SimpleDate::new(2014, 1, 1),
        SimpleDate::new(1999, 10, 6),
    ].to_vec();

    let bd_merkle_tree = BirthdayMerkleTree::new(&dates);
    let my_birthday = SimpleDate::new(1999, 10, 6);
    let proof = bd_merkle_tree.generate_path(15);

//*************************************YOUR CODE ENDS HERE*************************************

    // Give the private input to the guest, including Waldo's location.
    let input = PrivateInput {
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Pass private values you need to prove 1) your given date is in the tree 2) you are over 18
        element: my_birthday,
        proof: proof,
        bd_merkle_tree: bd_merkle_tree.root(),

    //*************************************YOUR CODE ENDS HERE*************************************
    };

    // Make the ExecutorEnv, registering an io_callback to communicate
    // vector oracle data from the Merkle tree.
    let env = ExecutorEnv::builder()
        .write(&input)?
        .io_callback(SYS_VECTOR_ORACLE, bd_merkle_tree.vector_oracle_callback())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, BIRTHDAY_ELF).unwrap().receipt;

    // Save the receipt to disk so it can be sent to the verifier.
    fs::write(&args.receipt, bincode::serialize(&receipt).unwrap())?;
    fs::write(&args.m_r, bincode::serialize(&bd_merkle_tree.root()).unwrap())?;
    println!("Success! Saved the receipt and the Merkle tree root");

    Ok(())
}