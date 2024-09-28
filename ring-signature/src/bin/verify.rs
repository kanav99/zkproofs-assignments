use std::{error::Error, fs};

use k256::{
    ecdsa::{SigningKey, Signature, signature::{Signer,Verifier},VerifyingKey},
    EncodedPoint,
    SecretKey,
};
use rand_core::OsRng;
use risc0_zkvm::Receipt;
use ring_core::{
    Journal,
};
use ring_methods::RING_ID;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Load and verify the receipt file.
    let receipt: Receipt = bincode::deserialize(&fs::read("./receipt.bin")?)?;
    receipt.verify(RING_ID)?;

    let journal: Journal = receipt.journal.decode()?;
    println!("Message: {:?}", journal.message);
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Check the consistency of whatever signature received in Journal is the same as in the signature.bin

    //*************************************YOUR CODE ENDS HERE*************************************

    println!("Successfully verified the proof of signature");
    Ok(())
}