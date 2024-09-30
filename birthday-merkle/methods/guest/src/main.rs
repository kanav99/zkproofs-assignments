use risc0_zkvm::guest::env;
use birthday_core::{
    Journal, PrivateInput,
};

fn main() {
    // Read a Merkle proof from the host.
    let input: PrivateInput = env::read();

    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Add code to verify your given date exists in the tree and the age is over 18
    input.proof.verify(&input.bd_merkle_tree, &input.element);

    //*************************************YOUR CODE ENDS HERE*************************************

    // Commit the verified public information into the journal.
    let journal = Journal {
    //************************************YOUR CODE STARTS HERE************************************

    // TODO: Add the public information you need to verify for consistency with the proof
        bd_merkle_tree: input.bd_merkle_tree,

    //*************************************YOUR CODE ENDS HERE*************************************
    };
    env::commit(&journal);
}