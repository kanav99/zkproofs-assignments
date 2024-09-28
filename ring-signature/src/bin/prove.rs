use std::{error::Error, fs};

use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use rand_core::OsRng;
// use rand::prelude::*;
use risc0_zkvm::{default_prover, ExecutorEnv};
use ring_core::PrivateInput;
use ring_methods::RING_ELF;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Generate two sets of keys, the message, and a single signature.
    let signing_key0 = SigningKey::random(&mut OsRng);
    let signing_key1 = SigningKey::random(&mut OsRng);

    let msg = b"Hello, World!";
    let signature: Signature = signing_key0.sign(msg);
    //*************************************YOUR CODE ENDS HERE*************************************


    // Give the private input to the guest.
    let input = PrivateInput {
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Define the private values you need to prove your signature indeed verifies under one of those keys.
        encoded_verifying_key0: signing_key0.verifying_key().to_encoded_point(true),
        encoded_verifying_key1: signing_key1.verifying_key().to_encoded_point(true),
        signature: signature,
        message: msg.to_vec(),
    //*************************************YOUR CODE ENDS HERE*************************************
    };

    // Make the ExecutorEnv
    let env = ExecutorEnv::builder()
        .write(&input)?
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, RING_ELF).unwrap().receipt;

    // Save the receipt & signature to disk so it can be sent to the verifier.
    fs::write("./receipt.bin", bincode::serialize(&receipt).unwrap())?;
    // fs::write(&args.sig, bincode::serialize(&signature.to_vec()).unwrap())?;
    println!("Success! Saved the receipt & signature");

    Ok(())
}