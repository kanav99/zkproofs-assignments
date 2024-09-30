
#[macro_use]
extern crate merkle_light_derive;

pub mod birthday;
pub mod merkle;

use birthday::BirthdayMerkleTree;
use merkle::{Node, Proof};
use serde::{Deserialize, Serialize};
use crate::birthday::SimpleDate;

/// Private input values to the birthday search.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateInput {
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Define private values you need to prove that your birthday is in the given Merkle tree and you are over 18
    pub element: SimpleDate,
    pub proof: Proof<SimpleDate>,
    pub bd_merkle_tree: Node,

    //*************************************YOUR CODE ENDS HERE*************************************
}

/// Public journal values that will be committed by the birthday membership method.
#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Define public values you need to prove for the above
    pub bd_merkle_tree: Node,
    //*************************************YOUR CODE ENDS HERE*************************************
}