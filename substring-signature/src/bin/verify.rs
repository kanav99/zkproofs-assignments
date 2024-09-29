use std::{error::Error, fs};

use risc0_zkvm::Receipt;
use substring_core::{
    Journal,
};
use substring_methods::SUBSTRING_ID;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Load and verify the receipt file.
    let receipt: Receipt = bincode::deserialize(&fs::read("./receipt.bin")?)?;
    receipt.verify(SUBSTRING_ID)?;

    let journal: Journal = receipt.journal.decode()?;

    //TODO: Initialize your signature with what is read from signature.bin
    // = bincode::deserialize(&fs::read(&args.sig)?)?;
    
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Check the consistency of whatever signature received in Journal is the same as in the signature.bin

    //*************************************YOUR CODE ENDS HERE*************************************

    println!(
        "Successfully verified the proof of signature",
        
    );
    Ok(())
}