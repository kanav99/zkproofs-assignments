
use k256::{
    ecdsa::Signature,
    EncodedPoint,
};

use serde::{Deserialize, Serialize};


/// Private input values to used to prove the signature is for a message that satisfies a condition
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateInput {
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Define private values you need to prove that signature is valid
    pub encoded_verifying_key0: EncodedPoint,
    pub encoded_verifying_key1: EncodedPoint,
    pub signature: Signature,
    pub message: Vec<u8>,
    //*************************************YOUR CODE ENDS HERE*************************************
}

/// Public journal values that will be committed by signature validity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Define the public values you need to verify the consistency with the proof
    pub encoded_verifying_key0: EncodedPoint,
    pub encoded_verifying_key1: EncodedPoint,
    //*************************************YOUR CODE ENDS HERE*************************************
}