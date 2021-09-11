use parity_scale_codec::{Decode, Encode};
use sha2::{Digest, Sha256};
use std::convert::TryInto;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct BlockHeader {
    number: u64,
    prev_hash: [u8; 32],
    root_hash: [u8; 32],
}

impl BlockHeader {
    pub fn new(number: u64, prev_hash: [u8; 32], root_hash: [u8; 32]) -> Self {
        Self {
            number,
            prev_hash,
            root_hash,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        Sha256::new()
            .chain(self.encode())
            .finalize()
            .try_into()
            .map_err(|_| "Expected length of the array is 32")
            .unwrap()
    }
}

impl Block {
    pub fn new(number: u64, prev_hash: [u8; 32]) -> Block {
        let root_hash = [0; 32];
        Block {
            header: BlockHeader::new(number, prev_hash, root_hash),
        }
    }
}

pub struct Block {
    header: BlockHeader,
}

impl Block {
    pub fn hash(&self) -> &BlockHeader {
        &self.header
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    fn default_block_header() -> BlockHeader {
        BlockHeader::new(0, [1; 32], [2; 32])
    }

    #[test]
    fn block_header_ser_test() {
        let block = default_block_header();
        let serialized = block.encode();
        let expected_serialized = hex::decode("000000000000000001010101010101010101010101010101010101010101010101010101010101010202020202020202020202020202020202020202020202020202020202020202").unwrap();
        assert_eq!(serialized, expected_serialized);
    }

    #[test]
    fn block_header_de_test() {
        let block = default_block_header();
        let serialized = hex::decode("000000000000000001010101010101010101010101010101010101010101010101010101010101010202020202020202020202020202020202020202020202020202020202020202").unwrap();
        let deserialized = BlockHeader::decode(&mut serialized.as_ref()).unwrap();
        assert_eq!(block, deserialized);
    }

    #[test]
    fn block_header_serde_test() {
        let block = default_block_header();
        let serialized = block.encode();
        let deserialized = BlockHeader::decode(&mut serialized.as_ref()).unwrap();
        assert_eq!(block, deserialized);
    }

    #[test]
    fn block_header_hash_test() {
        let block = default_block_header();
        let expected_hash: [u8; 32] =
            hex::decode("0aa07d23daebf0b061e7f4dd96908797c7e2078cc8d0abd172b9b33a873114b2")
                .unwrap()
                .try_into()
                .unwrap();
        assert_eq!(block.hash(), expected_hash);
    }
}
