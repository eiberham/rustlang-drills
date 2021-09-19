use sha2::{Sha256, Digest};
use serde;

type Hash = Vec<u8>;

#[derive(Debug)]
pub struct Payload {
    pub amount: u64,
    pub receiver: String,
    pub sender: String,
}

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub nonce: u64,
    pub hash: Hash,
    pub payload: Payload,
    pub previous_hash: Hash,
    pub timestamp: u64
}

impl Block {
  pub fn new(index: u32, nonce: u64, hash: Hash, payload: Payload, previous_hash: Hash, timestamp: u64) -> Self {
    Block {
      index,
      nonce,
      hash: vec![0; 32],
      payload,
      previous_hash,
      timestamp
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