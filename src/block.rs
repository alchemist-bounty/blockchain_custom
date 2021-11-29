use std::fmt::{ self, Debug, Formatter };
use super::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u32,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{index}]: {} at: {} with: {}",
            &self.index,
            &self.hash,
            &self.timestamp,
            &self.payload,
        )
    }
}

impl Block {
    pub fn new( index: u32, timestamp: u32, hash: BlockHash, prev_block_hash: BlockHash, nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}
 