use sha2::{Sha256, Digest};
use serde;

pub struct Data {
    amount: usize,
    receiver: String,
    sender: String,
}

pub struct Block {
    pub data: Data,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: usize
}

impl Block {
  pub fn new() -> Block {
    Block {

    }
  }

  pub fn calculate_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    // write input message
    hasher.update(data);
    // read hash digest and consume hasher
    // {:x} means format as hexadecimal using the std::fmt::UpperHex trait, which is implemented for GenericArray.
    let hash : String = format!("{:X}", hasher.finalize());
    hash
  }
}