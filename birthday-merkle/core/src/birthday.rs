use serde::{Deserialize, Serialize};

use crate::merkle::{MerkleTree, Node, Proof};


#[derive(Debug, Clone, Serialize, Deserialize, Hashable)]
pub struct SimpleDate{
    y: u16,
    m: u8,
    d: u8,
}
impl SimpleDate {
    pub fn new(y: u16, m: u8, d: u8) -> SimpleDate {
        SimpleDate { y: y, m: m, d:d }
    }
}
/// BirthdayMerkleTree is a merklization of a list of dates, constructed with leaf
/// elements, traversed in left-to-right and top-to-bottom
/// order.
///
pub struct BirthdayMerkleTree(MerkleTree<SimpleDate>);

impl BirthdayMerkleTree {
    pub fn new(dates :& Vec<SimpleDate>) -> Self {
        Self(MerkleTree::new(dates.clone()))
    }

    pub fn root(&self) -> Node {
        self.0.root()
    }

    #[cfg(not(target_os = "zkvm"))]
    pub fn vector_oracle_callback(
        &self,
    ) -> impl Fn(risc0_zkvm::Bytes) -> risc0_zkvm::Result<risc0_zkvm::Bytes> + '_ {
        self.0.vector_oracle_callback()
    }

    pub fn generate_path(&self, i: usize) -> Proof<SimpleDate> {
        self.0.generate_path(i)
    }
}

#[cfg(target_os = "zkvm")]
mod zkvm {
    use crate::merkle::{Node, VectorOracle};
    use super::SimpleDate;
    pub struct BirthdayOracle {
        dates : VectorOracle<SimpleDate>,
    }

    impl BirthdayOracle {
        pub fn new(root: Node) -> Self {
            Self {
                dates: VectorOracle::new(root),
            }
        }

        pub fn root(&self) -> &Node {
            self.dates.root()
        }
    }
}

#[cfg(target_os = "zkvm")]
pub use crate::birthday::zkvm::*;