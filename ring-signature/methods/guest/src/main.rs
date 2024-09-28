use k256::ecdsa::{signature::Verifier, VerifyingKey};
use risc0_zkvm::guest::env;
use ring_core::{Journal, PrivateInput};

fn main() {
    
    let input: PrivateInput = env::read();

   //************************************YOUR CODE STARTS HERE************************************

    // TODO: Add code to verify the signature indeed verifies for one of the keys

    let verifying_key0 = VerifyingKey::from_encoded_point(&input.encoded_verifying_key0).unwrap();
    let verifying_key1 = VerifyingKey::from_encoded_point(&input.encoded_verifying_key1).unwrap();

    let message = &input.message;
    let signature = input.signature;

    let b0 = verifying_key0.verify(&message, &signature);
    let b1 = verifying_key1.verify(&message, &signature);

    let t = b0.is_ok() || b1.is_ok();
    assert!(t);

    //*************************************YOUR CODE ENDS HERE*************************************

    let journal = Journal{
    //************************************YOUR CODE STARTS HERE************************************
    
    // TODO: Add values that will be part of public verification
        encoded_verifying_key0: input.encoded_verifying_key0,
        encoded_verifying_key1: input.encoded_verifying_key1,
        message: input.message,

    //*************************************YOUR CODE ENDS HERE*************************************
    };
    
    env::commit(&journal);
}